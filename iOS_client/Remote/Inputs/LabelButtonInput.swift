//
//  LabelButtonInput.swift
//  Remote
//
//  Created by Indiana Kernick on 30/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class LabelButtonInput: UILabel {
    static private let downColor = Colors.gray500
    static private let upColor = Colors.gray700
    
    private var firstTouch: UITouch?
    
    // --- LabelButtonInput --- //
    
    var pressed = {}
    var released = {}
    
    // --- UIView --- //
    
    override func layoutSubviews() {
        isMultipleTouchEnabled = false
        isUserInteractionEnabled = true
        layer.masksToBounds = true
        layer.cornerRadius = 8
        layer.backgroundColor = LabelButtonInput.upColor
    }
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard firstTouch == nil, let touch = touches.first else {
            return super.touchesBegan(touches, with: event)
        }
        firstTouch = touch
        layer.backgroundColor = LabelButtonInput.downColor
        pressed()
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesEnded(touches, with: event)
        }
        firstTouch = nil
        layer.backgroundColor = LabelButtonInput.upColor
        released()
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesCancelled(touches, with: event)
        }
        firstTouch = nil
        layer.backgroundColor = LabelButtonInput.upColor
        released()
    }
}
