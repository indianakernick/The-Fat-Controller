//
//  ButtonInput.swift
//  Remote
//
//  Created by Indiana Kernick on 5/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class ButtonInput: UIButton {
    static private let downColor = Colors.gray500
    static private let upColor = Colors.gray700
    
    private var initialized = false
    
    @objc private func touchDown() {
        layer.backgroundColor = ButtonInput.downColor
    }
    
    @objc private func touchUp() {
        layer.backgroundColor = ButtonInput.upColor
    }
    
    // --- UIView --- //
    
    override func layoutSubviews() {
        super.layoutSubviews()
        if initialized {
            return
        }
        initialized = true
        layer.masksToBounds = true
        layer.cornerRadius = 8
        layer.backgroundColor = ButtonInput.upColor
        addTarget(self, action: #selector(touchDown), for: .touchDown)
        addTarget(self, action: #selector(touchUp), for: .touchUpInside)
        addTarget(self, action: #selector(touchUp), for: .touchUpOutside)
        addTarget(self, action: #selector(touchUp), for: .touchCancel)
    }
}
