//
//  NewButtonInput.swift
//  Remote
//
//  Created by Indiana Kernick on 5/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class NewButtonInput: UIButton {
    static private let downColor = Colors.gray500
    static private let upColor = Colors.gray700
    
    // layoutSubviews seems to be called many times
    private var initialized = false
    
    override func layoutSubviews() {
        super.layoutSubviews()
        if initialized {
            return
        }
        initialized = true
        layer.masksToBounds = true
        layer.cornerRadius = 8
        layer.backgroundColor = NewButtonInput.upColor
        addTarget(self, action: #selector(touchDown), for: .touchDown)
        addTarget(self, action: #selector(touchUp), for: .touchUpInside)
        addTarget(self, action: #selector(touchUp), for: .touchUpOutside)
        addTarget(self, action: #selector(touchUp), for: .touchCancel)
    }
    
    @objc private func touchDown() {
        layer.backgroundColor = NewButtonInput.downColor
    }
    
    @objc private func touchUp() {
        layer.backgroundColor = NewButtonInput.upColor
    }
}
