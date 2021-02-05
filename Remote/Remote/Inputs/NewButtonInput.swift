//
//  NewButtonInput.swift
//  Remote
//
//  Created by Indiana Kernick on 5/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;

class NewButtonInput: UIButton {
    static private let downColor = Colors.gray600;
    static private let upColor = Colors.gray700;
    
    var pressed = {};
    var released = {};
    
    override func layoutSubviews() {
        super.layoutSubviews();
        layer.masksToBounds = true;
        layer.cornerRadius = 8;
        layer.backgroundColor = NewButtonInput.upColor;
        addTarget(self, action: #selector(touchDown), for: .touchDown);
        addTarget(self, action: #selector(touchUp), for: .touchUpInside);
        addTarget(self, action: #selector(touchUp), for: .touchUpOutside);
    }
    
    override open var isHighlighted: Bool {
        didSet {
            layer.backgroundColor = isHighlighted ? NewButtonInput.downColor : NewButtonInput.upColor;
        }
    }
    
    @objc private func touchDown() {
        pressed();
    }
    
    @objc private func touchUp() {
        released();
    }
}
