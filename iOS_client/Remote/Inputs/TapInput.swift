//
//  TapInput.swift
//  Remote
//
//  Created by Indiana Kernick on 2/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class TapInput: UIView {
    static private let downColor = Colors.gray500
    static private let upColor = Colors.gray700
    
    private var firstTouch: UITouch?
    
    // --- TapInput --- //
    
    var pressed = {}
    var released = {}
    
    // --- UIView --- //
    
    override func layoutSubviews() {
        layer.backgroundColor = TapInput.upColor
    }
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard firstTouch == nil, let touch = touches.first else {
            return super.touchesBegan(touches, with: event)
        }
        firstTouch = touch
        layer.backgroundColor = TapInput.downColor
        pressed()
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesEnded(touches, with: event)
        }
        firstTouch = nil
        layer.backgroundColor = TapInput.upColor
        released()
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first, firstTouch == touch else {
            return super.touchesCancelled(touches, with: event)
        }
        firstTouch = nil
        layer.backgroundColor = TapInput.upColor
        released()
    }
}
