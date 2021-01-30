//
//  VolumeEmitter.swift
//  Remote
//
//  Created by Indiana Kernick on 30/1/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

class OnOffVolumeEmitter: VolumeInputDelegate {
    private var socket: SocketManager;
    
    var volumeUpOn = [UInt8]();
    var volumeUpOff = [UInt8]();
    var volumeDownOn = [UInt8]();
    var volumeDownOff = [UInt8]();
    
    init(socket: SocketManager) {
        self.socket = socket;
    }
    
    func volumeUpPressed() {
        socket.send(volumeUpOn);
    }
    
    func volumeUpReleased() {
        socket.send(volumeUpOff);
    }
    
    func volumeDownPressed() {
        socket.send(volumeDownOn);
    }
    
    func volumeDownReleased() {
        socket.send(volumeDownOff);
    }
}

class SwitchVolumeEmitter: VolumeInputDelegate {
    private var socket: SocketManager;
    private var up: Bool;
    
    var volumeUp = [UInt8]();
    var volumeDown = [UInt8]();
    
    init(socket sock: SocketManager, initiallyUp: Bool) {
        socket = sock;
        up = initiallyUp;
    }
    
    func volumeUpPressed() {
        if !up {
            up = true;
            socket.send(volumeUp);
        }
    }
    
    func volumeDownPressed() {
        if up {
            up = false;
            socket.send(volumeDown);
        }
    }
    
    func volumeUpReleased() {}
    func volumeDownReleased() {}
}
