//
//  TextInput.swift
//  Remote
//
//  Created by Indiana Kernick on 26/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class TextInput: UITextField {
    func setIndent(_ indent: CGFloat) {
        let left = UIView(frame: CGRect(
            x: 0, y: 0,
            width: indent, height: frame.size.height
        ))
        left.backgroundColor = backgroundColor
        leftView = left
        leftViewMode = .always
    }
}
