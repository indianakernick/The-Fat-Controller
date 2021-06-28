//
//  MultilineTextInputCell.swift
//  Remote
//
//  Created by Indiana Kernick on 28/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit

class MultilineTextInputCell: UITableViewCell, UITextViewDelegate {
    @IBOutlet weak var textInput: UITextView!
    
    // --- MultilineTextInputCell --- //
    
    var textChanged: (String) -> Void = { text in }
    
    // --- UITableViewCell --- //
    
    override func layoutSubviews() {
        super.layoutSubviews()
        textInput.delegate = self
        textInput.textContainerInset = UIEdgeInsets.zero
        textInput.textContainer.lineFragmentPadding = 0
        textInput.isScrollEnabled = false
        textViewDidChange(textInput)
    }
    
    // --- UITextViewDelegate --- //
    
    func textViewDidChange(_ textView: UITextView) {
        let textHeight = textView.sizeThatFits(textView.frame.size).height
        let rowHeight = textHeight + layoutMargins.top + layoutMargins.bottom
        // sizeThatFits(CGSize.zero).height returns 43.5
        // Where can I get 44?
        let minimumHeight = CGFloat(44)
        
        if rowHeight < minimumHeight {
            textView.textContainerInset.top = 3
            frame.size.height = minimumHeight
        } else {
            textView.textContainerInset.top = 0
            frame.size.height = rowHeight
        }
        
        textChanged(textView.text ?? "")
    }
}
