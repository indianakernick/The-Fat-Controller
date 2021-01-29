//
//  LookView.swift
//  Remote
//
//  Created by Indiana Kernick on 28/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

protocol LookInputDelegate: class {
    func lookDirectionChanged(dx: Int32, dy: Int32);
}

class LookView: UIView {
    private var velXLabel = UILabel(frame: CGRect(x: 10.0, y: 10.0, width: 300, height: 20));
    private var velYLabel = UILabel(frame: CGRect(x: 10.0, y: 30.0, width: 300, height: 20));
    
    private var firstTouch: UITouch?;
    private var lastLocation = CGPoint();
    private var lastTimestamp = TimeInterval();
    private var accumulator = CGPoint();
    
    weak var delegate: LookInputDelegate?;
    var slowScale = CGFloat(1.0);
    var middleScale = CGFloat(1.0);
    var fastScale = CGFloat(1.0);
    var slowVelocity = CGFloat(1.0);
    var fastVelocity = CGFloat(1.0);
    
    override func layoutSubviews() {
        layer.backgroundColor = CGColor(srgbRed: 0.0, green: 0.0, blue: 1.0, alpha: 1.0);
        addSubview(velXLabel);
        addSubview(velYLabel);
    }
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard firstTouch == nil, let touch = touches.first else {
            return super.touchesBegan(touches, with: event);
        }
        firstTouch = touch;
        lastLocation = touch.preciseLocation(in: self);
        lastTimestamp = event!.timestamp;
    }
    
    private func applyCurve(dpos: CGFloat, vel: CGFloat) -> CGFloat {
        let absVel = abs(vel);
        if absVel < slowVelocity {
            return dpos * slowScale;
        } else if absVel < fastVelocity {
            return dpos * middleScale;
        } else {
            return dpos * fastScale;
        }
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesMoved(touches, with: event);
        }
        
        let location = touch.preciseLocation(in: self);
        let timestamp = event!.timestamp;
        let dx = location.x - lastLocation.x;
        let dy = location.y - lastLocation.y;
        let dt = CGFloat(timestamp - lastTimestamp);
        let vx = dx / dt;
        let vy = dy / dt;
        
        velXLabel.text = String(format: "%f", round(vx / 10) * 10);
        velYLabel.text = String(format: "%f", round(vy / 10) * 10);
        
        let dxScale = accumulator.x + applyCurve(dpos: dx, vel: vx);
        let dyScale = accumulator.y + applyCurve(dpos: dy, vel: vy);
        let dxTrunc = trunc(dxScale);
        let dyTrunc = trunc(dyScale);
        let dxInt = Int32(dxTrunc);
        let dyInt = Int32(dyTrunc);
        
        accumulator.x = dxScale - dxTrunc;
        accumulator.y = dyScale - dyTrunc;
        
        if dxInt != 0 || dyInt != 0 {
            delegate?.lookDirectionChanged(
                dx: dxInt,
                dy: dyInt
            );
        }
        lastLocation = location;
        lastTimestamp = timestamp;
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesEnded(touches, with: event);
        }
        firstTouch = nil;
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesCancelled(touches, with: event);
        }
        firstTouch = nil;
    }
}
