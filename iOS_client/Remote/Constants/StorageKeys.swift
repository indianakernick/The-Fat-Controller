//
//  StorageKeys.swift
//  Remote
//
//  Created by Indiana Kernick on 3/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

enum StorageKeys {
    static let tapUpCommandList = "TapCommandListUp"
    static let tapDownCommandList = "TapCommandListDown"
    static let hostName = "HostName"
    static let lowLatencyMode = "LowLatencyMode"
    
    static func setHostName(_ value: String) {
        UserDefaults.standard.setValue(value, forKey: hostName)
    }
    
    static func getHostName() -> String {
        UserDefaults.standard.string(forKey: hostName) ?? ""
    }
    
    static func setLowLatencyMode(_ value: Bool) {
        UserDefaults.standard.setValue(value, forKey: lowLatencyMode)
    }
    
    static func getLowLatencyMode() -> Bool {
        // bool(forKey:) returns false if the key doesn't exist. I want low
        // latency mode to be on by default.
        UserDefaults.standard.object(forKey: lowLatencyMode) as? Bool ?? true
    }
}
