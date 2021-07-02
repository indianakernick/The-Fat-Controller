//
//  ScannerVC.swift
//  Remote
//
//  Created by Indiana Kernick on 2/7/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit
import CryptoKit
import AVFoundation

// https://www.hackingwithswift.com/example-code/media/how-to-scan-a-qr-code

class ScannerVC: UIViewController, AVCaptureMetadataOutputObjectsDelegate {
    private var captureSession: AVCaptureSession!
    private var previewLayer: AVCaptureVideoPreviewLayer!
    
    private func failed() {
        DispatchQueue.main.async {
            let alert = UIAlertController(
                title: "Scanning not supported",
                message: "This device does not support scanning QR codes.",
                preferredStyle: .alert
            )
            let action = UIAlertAction(title: "OK", style: .default) { action in
                self.dismiss(animated: true)
            }
            alert.addAction(action)
            self.present(alert, animated: true)
        }
        captureSession = nil
    }
    
    // --- ScannerVC --- //
    
    var succeeded: (SymmetricKey) -> Void = { key in }
    
    // --- UIViewController --- //
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        captureSession = AVCaptureSession()
        
        // Configure the video input
        
        guard let videoCaptureDevice = AVCaptureDevice.default(for: .video) else { return }
        let videoInput: AVCaptureDeviceInput
        
        do {
            videoInput = try AVCaptureDeviceInput(device: videoCaptureDevice)
        } catch {
            print(error)
            return
        }
        
        if captureSession.canAddInput(videoInput) {
            captureSession.addInput(videoInput)
        } else {
            failed()
            return
        }
        
        // Configure the QR code output
        
        let metadataOutput = AVCaptureMetadataOutput()
        
        if captureSession.canAddOutput(metadataOutput) {
            captureSession.addOutput(metadataOutput)
            metadataOutput.setMetadataObjectsDelegate(self, queue: DispatchQueue.main)
            metadataOutput.metadataObjectTypes = [.qr]
        } else {
            failed()
            return
        }
        
        // Configure video preview
        
        previewLayer = AVCaptureVideoPreviewLayer(session: captureSession)
        previewLayer.frame = view.layer.bounds
        previewLayer.videoGravity = .resizeAspectFill
        view.layer.addSublayer(previewLayer)
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        if captureSession?.isRunning == false {
            captureSession.startRunning()
        }
    }
    
    override func viewDidDisappear(_ animated: Bool) {
        super.viewDidDisappear(animated)
        if captureSession?.isRunning == true {
            captureSession.stopRunning()
        }
    }
    
    override var prefersStatusBarHidden: Bool {
        return true
    }

    override var supportedInterfaceOrientations: UIInterfaceOrientationMask {
        return .portrait
    }
    
    // --- AVCaptureMetadataOutputObjectsDelegate --- //
    
    func metadataOutput(_ output: AVCaptureMetadataOutput, didOutput metadataObjects: [AVMetadataObject], from connection: AVCaptureConnection) {
        for metadataObject in metadataObjects {
            guard let readableObject = metadataObject as? AVMetadataMachineReadableCodeObject else { continue }
            guard let stringValue = readableObject.stringValue else { continue }
            guard let decodedData = Data(base64Encoded: stringValue) else { continue }
            if decodedData.count != SocketManager.keyLength { continue }
            
            captureSession.stopRunning()
            succeeded(SymmetricKey(data: decodedData))
            Haptic.strongTap()
            dismiss(animated: true)
            return
        }
    }
}
