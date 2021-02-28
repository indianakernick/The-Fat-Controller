//
//  StorageKeys.swift
//  Remote
//
//  Created by Indiana Kernick on 3/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

enum StorageKeys {
    static let tapUpCommandList = "TapCommandListUp"
    static let tapDownCommandList = "TapCommandListDown"
    static let hostAddress = "HostAddress"
    static let hostPort = "HostPort"
    static let selectedTabIndex = "TabBarSelectedIndex"
}

enum StorageDefaults {
    static let hostPort: UInt16 = 2048
}
