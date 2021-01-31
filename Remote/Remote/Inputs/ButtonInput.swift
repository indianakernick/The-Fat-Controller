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

class ButtonInput: UILabel {
    static private let downColor = Colors.gray500;
    static private let upColor = Colors.gray700;
    
    private var firstTouch: UITouch?;
    
    weak var delegate: ButtonInputDelegate?;
    var button = Button.a;
    
    override func layoutSubviews() {
        isMultipleTouchEnabled = false;
        isUserInteractionEnabled = true;
        layer.masksToBounds = true;
        layer.cornerRadius = 8;
        layer.backgroundColor = ButtonInput.upColor;
    }
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard firstTouch == nil, let touch = touches.first else {
            return super.touchesBegan(touches, with: event);
        }
        firstTouch = touch;
        layer.backgroundColor = ButtonInput.downColor;
        delegate?.buttonPressed(button: button);
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesEnded(touches, with: event);
        }
        firstTouch = nil;
        layer.backgroundColor = ButtonInput.upColor;
        delegate?.buttonReleased(button: button);
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesCancelled(touches, with: event);
        }
        firstTouch = nil;
        layer.backgroundColor = ButtonInput.upColor;
        delegate?.buttonReleased(button: button);
    }
}
