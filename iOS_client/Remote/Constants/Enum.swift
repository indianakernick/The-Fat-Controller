//
//  Enum.swift
//  Remote
//
//  Created by Indiana Kernick on 26/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

protocol Enum: CaseIterable & CustomStringConvertible & RawRepresentable where RawValue == UInt8 {}
