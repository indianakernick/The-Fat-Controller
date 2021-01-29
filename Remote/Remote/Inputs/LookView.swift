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
    private var firstTouch: UITouch?;
    private var lastLocation = CGPoint();
    
    weak var delegate: LookInputDelegate?;
    var scale = CGFloat(1.0);
    
    override func layoutSubviews() {
        layer.backgroundColor = CGColor(srgbRed: 0.0, green: 0.0, blue: 1.0, alpha: 1.0);
    }
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard firstTouch == nil, let touch = touches.first else {
            return super.touchesBegan(touches, with: event);
        }
        firstTouch = touch;
        lastLocation = touch.location(in: self);
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesMoved(touches, with: event);
        }
        let location = touch.location(in: self);
        delegate?.lookDirectionChanged(
            dx: Int32(round(scale * (location.x - lastLocation.x))),
            dy: Int32(round(scale * (location.y - lastLocation.y)))
        );
        lastLocation = location;
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
