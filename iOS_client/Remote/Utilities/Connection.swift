//
//  Connection.swift
//  Remote
//
//  Created by Indiana Kernick on 15/3/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import Network
import Foundation

protocol ConnectionDelegate: class {
    func connectionGained()
    func connectionLost()
}

// A wrapper for NWConnection
class Connection {
    private let queue = DispatchQueue(label: "Queue")
    private var handle: NWConnection!
    private var currHost: NWEndpoint.Host!
    private var currPort: NWEndpoint.Port!
    private var connectionLost = true
    
    weak var delegate: ConnectionDelegate?
    
    func connect(host: String, port: UInt16) {
        assert(!host.isEmpty)
        currHost = NWEndpoint.Host(host)
        currPort = NWEndpoint.Port(integerLiteral: port)
        connect()
    }
    
    func connect() {
        // Need to ensure that the old connection is closed. If we don't do
        // this, the FIN packet won't be sent and the server won't know that
        // we've disconnected.
        if handle != nil {
            handle.cancel()
        }
        handle = NWConnection(host: currHost, port: currPort, using: .tcp)
        handle.stateUpdateHandler = stateChanged
        receive()
        handle.start(queue: queue)
    }
    
    func send(_ data: Data) {
        handle.send(content: data, completion: .contentProcessed({ error in
            if let error = error {
                print(error)
            }
        }))
    }
    
    func isDisconnected() -> Bool {
        let state = handle.state
        return state != .setup && state != .preparing && state != .ready
    }
    
    private func stateChanged(to: NWConnection.State) {
        switch to {
        case .setup:
            break
        case .preparing:
            break
        case .ready:
            delegate?.connectionGained()
        case .waiting(_):
            fallthrough
        case .failed(_):
            delegate?.connectionLost()
        case .cancelled:
            break
        @unknown default:
            break
        }
    }
    
    // This is the TCP client read() loop. The framework calls read() in another
    // thread and then calls this completion handler whenever something is
    // returned. We only care about errors and whether the connection has
    // closed.
    private func receive() {
        let receiveHandle = handle
        handle.receive(minimumIncompleteLength: 1, maximumLength: 65536) {
            (_, _, isComplete, error) in
            // There might be a completion handler on the queue that was created
            // for an old handle so ignore it.
            if receiveHandle !== self.handle {
                return
            }
            // isComplete means that the connection has closed normally.
            if isComplete || error != nil {
                self.handle.cancel()
                self.delegate?.connectionLost()
            } else {
                // handle.receive pushes the lambda onto the dispatch queue.
                // Once the lambda has been removed, we add another one.
                self.receive()
            }
        }
    }
}
