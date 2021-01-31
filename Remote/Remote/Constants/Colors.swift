//
//  Colors.swift
//  Remote
//
//  Created by Indiana Kernick on 29/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import CoreGraphics;

fileprivate func hexColor(_ hex: UInt32) -> CGColor {
    let red = (hex >> 16) & 0xff;
    let green = (hex >> 8) & 0xff;
    let blue = hex & 0xFF;
    let array = [CGFloat(red) / 255.0, CGFloat(green) / 255.0, CGFloat(blue) / 255.0, CGFloat(1.0)];
    return array.withUnsafeBufferPointer() { pointer in
        return CGColor(colorSpace: CGColorSpace(name: CGColorSpace.extendedSRGB)!, components: pointer.baseAddress!)!;
    };
    /*
    return CGColor(
        srgbRed: CGFloat(red) / 255.0,
        green: CGFloat(green) / 255.0,
        blue: CGFloat(blue) / 255.0,
        alpha: 1.0
    );
    */
}

enum Colors {
    // Color scheme taken from Bootstrap.
    // I really like the Bootstrap colors!
    
    static let white = hexColor(0xffffff);
    static let gray100 = hexColor(0xf8f9fa);
    static let gray200 = hexColor(0xe9ecef);
    static let gray300 = hexColor(0xdee2e6);
    static let gray400 = hexColor(0xced4da);
    static let gray500 = hexColor(0xadb5bd);
    static let gray600 = hexColor(0x6c757d);
    static let gray700 = hexColor(0x495057);
    static let gray800 = hexColor(0x343a40);
    static let gray900 = hexColor(0x212529);
    static let black = hexColor(0x000000);
    
    static let blue = hexColor(0x007bff);
    static let indigo = hexColor(0x6610f2);
    static let purple = hexColor(0x6f42c1);
    static let pink = hexColor(0xe83e8c);
    static let red = hexColor(0xdc3545);
    static let orange = hexColor(0xfd7e14);
    static let yellow = hexColor(0xffc107);
    static let green = hexColor(0x28a745);
    static let teal = hexColor(0x20c997);
    static let cyan = hexColor(0x17a2b8);
}
