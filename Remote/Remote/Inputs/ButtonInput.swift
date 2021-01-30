//
//  ButtonInput.swift
//  Remote
//
//  Created by Indiana Kernick on 30/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;

// Maybe this should be a plain integer ID instead of an enum?
// I'm not sure if either is right.
enum Button {
    // Hmmm...
    case volumeUp, volumeDown;
    case a, b, c, d, e, f;
}

protocol ButtonInputDelegate: class {
    func buttonPressed(button: Button);
    func buttonReleased(button: Button);
}

class ButtonInput: UIView {
    private var firstTouch: UITouch?;
    
    weak var delegate: ButtonInputDelegate?;
    var color = CGColor(genericGrayGamma2_2Gray: 1.0, alpha: 1.0);
    var button = Button.a;
    
    override func layoutSubviews() {
        layer.backgroundColor = color;
        isMultipleTouchEnabled = false;
    }
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard firstTouch == nil, let touch = touches.first else {
            return super.touchesBegan(touches, with: event);
        }
        firstTouch = touch;
        delegate?.buttonPressed(button: button);
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesEnded(touches, with: event);
        }
        firstTouch = nil;
        delegate?.buttonReleased(button: button);
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesCancelled(touches, with: event);
        }
        firstTouch = nil;
        delegate?.buttonReleased(button: button);
    }
}
