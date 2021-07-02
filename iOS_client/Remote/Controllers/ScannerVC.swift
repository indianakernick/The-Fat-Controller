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

protocol ScannerDelegate: AnyObject {
    func scanDidSucceed(key: SymmetricKey)
    func scanDidFail()
    func scanWasCancelled()
}

class ScannerVC: UIViewController, AVCaptureMetadataOutputObjectsDelegate {
    private var captureSession: AVCaptureSession!
    private var previewLayer: AVCaptureVideoPreviewLayer!
    
    // --- ScannerVC --- //
    
    weak var delegate: ScannerDelegate? = nil
    
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
            delegate?.scanDidFail()
            return
        }
        
        // Configure the QR code output
        
        let metadataOutput = AVCaptureMetadataOutput()
        
        if captureSession.canAddOutput(metadataOutput) {
            captureSession.addOutput(metadataOutput)
            metadataOutput.setMetadataObjectsDelegate(self, queue: DispatchQueue.main)
            metadataOutput.metadataObjectTypes = [.qr]
        } else {
            delegate?.scanDidFail()
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
        captureSession.startRunning()
    }
    
    override func viewDidDisappear(_ animated: Bool) {
        super.viewDidDisappear(animated)
        captureSession.stopRunning()
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
            delegate?.scanDidSucceed(key: SymmetricKey(data: decodedData))
            Haptic.strongTap()
            dismiss(animated: true)
            return
        }
    }
}
