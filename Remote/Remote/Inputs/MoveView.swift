//
//  MoveView.swift
//  Remote
//
//  Created by Indiana Kernick on 28/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

enum MoveDirection: UInt8 {
    case none = 0;
    case up = 1;
    case right = 2;
    case down = 4;
    case left = 8;
    // I seriously can't write `case upRight = 1 | 3`?
    // wtf
    case upRight = 3; // 1 | 3
    case downRight = 6; // 2 | 4
    case downLeft = 12; // 4 | 8
    case upLeft = 9; // 1 | 8
}

enum MoveOrigin {
    case absolute, relative;
}

protocol MoveInputDelegate: class {
    func moveDirectionChanged(old: MoveDirection, new: MoveDirection);
}

class MoveView: UIView {
    private var firstTouch: UITouch?;
    private var startLocation = CGPoint();
    private var lastDirection = MoveDirection.none;
    
    weak var delegate: MoveInputDelegate?;
    var origin = MoveOrigin.absolute;
    var stationaryThreshold = CGFloat(1.0);
    
    override func layoutSubviews() {
        layer.backgroundColor = CGColor(srgbRed: 1.0, green: 0.0, blue: 0.0, alpha: 1.0);
    }
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard firstTouch == nil, let touch = touches.first else {
            return super.touchesBegan(touches, with: event);
        }
        firstTouch = touch;
        let location = touch.location(in: self);
        switch origin {
        case .absolute:
            startLocation = center;
            setDirection(new: getDirection(location: location));
        case .relative:
            startLocation = location;
        }
    }
    
    private func getDirection(location: CGPoint) -> MoveDirection {
        let dx =  location.x - startLocation.x;
        let dy =  location.y - startLocation.y;
        if dx * dx + dy * dy < stationaryThreshold * stationaryThreshold {
            return .none;
        }
        let sixteenth = CGFloat.pi / 8.0;
        let eighth = CGFloat.pi / 4.0;
        let angle = atan2(dy, dx) + sixteenth;
        if 0.0 <= angle && angle <= eighth {
            return .right;
        } else if eighth < angle && angle < 2.0 * eighth {
            return .downRight;
        } else if 2.0 * eighth <= angle && angle <= 3.0 * eighth {
            return .down;
        } else if 3.0 * eighth < angle && angle < 4.0 * eighth {
            return .downLeft;
        } else if -eighth < angle && angle < 0.0 {
            return .upRight;
        } else if 2.0 * -eighth <= angle && angle <= -eighth {
            return .up;
        } else if 3.0 * -eighth < angle && angle < 2.0 * eighth {
            return .upLeft;
        } else {
            return .left;
        }
    }
    
    private func setDirection(new direction: MoveDirection) {
        if direction != lastDirection {
            delegate?.moveDirectionChanged(old: lastDirection, new: direction);
            lastDirection = direction;
        }
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesMoved(touches, with: event);
        }
        setDirection(new: getDirection(location: touch.location(in: self)));
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesEnded(touches, with: event);
        }
        setDirection(new: .none);
        firstTouch = nil;
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesCancelled(touches, with: event);
        }
        setDirection(new: .none);
        firstTouch = nil;
    }
}
