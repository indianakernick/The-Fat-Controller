//
//  Storage.swift
//  Remote
//
//  Created by Indiana Kernick on 3/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Foundation

enum Storage {
    private static let hostName = "HostName"
    private static let lowLatencyMode = "LowLatencyMode"
    private static let tapUpCommandList = "TapCommandListUp"
    private static let tapDownCommandList = "TapCommandListDown"
    
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
    
    // I don't remember why I'm using set here and setValue elsewhere.
    // I'm not sure what the difference is.
    
    static func setTapUpCommandList(_ value: [Any]) {
        UserDefaults.standard.set(value, forKey: tapUpCommandList)
    }
    
    static func getTapUpCommandList() -> [Any]? {
        UserDefaults.standard.array(forKey: tapUpCommandList)
    }
    
    static func setTapDownCommandList(_ value: [Any]) {
        UserDefaults.standard.set(value, forKey: tapDownCommandList)
    }
    
    static func getTapDownCommandList() -> [Any]? {
        UserDefaults.standard.array(forKey: tapDownCommandList)
    }
}
