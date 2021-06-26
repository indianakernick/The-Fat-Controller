//
//  CommandVC.swift
//  Remote
//
//  Created by Indiana Kernick on 25/6/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

// It's beginning to feel like the storyboard is more of a hinderence than a
// help. Using the storyboard makes it difficult to reuse similar things and
// parametize things. This PickerVC should be parametized but the fact that an
// instance of it is created by "the storyboard thing" makes that kind of
// clunky. I have to define things in the storyboard then tweak them in code. It
// would be much nicer to just do everything in code. Passing parameters down to
// child components is a breeze with Vue. The ability to do that easily is what
// Vue is all about.

// I run into a lot of problems with the storyboard file. When I rename a class
// (using the refactor tool), I expect that change to not break the storyboard
// file but of course it does. You're not supposed to edit the storyboard file
// directly. You're supposed to use the clunky interface builder. It was clunky
// when I started using it, and it's still clunky.

// The thing is that I think I'm in too deep with the storyboards to go back
// now. I'd be doing a lot of things from scratch. This app has gotten much more
// complicated than I initially intended.

// UIKit as a whole is a bit annoying to use. For example, putting a UITextField
// inside of a UITableViewCell requires me to manually indent the text. That's
// just dumb. Also, the default text size of table cells is a few points smaller
// than it should be so I have to manually bump it up to 14 pt. I also having to
// manually set the text color of the detail text label to the secondary label
// color. I would have expected the defaults to just do the right thing but that
// doesn't seem to be the case.

// The number keyboard doesn't have a minus sign. So you have to make your own
// number keyboard of cobble something together. There are a lot of these silly
// things that I have to deal with. I keep attributing these problems to my
// inexperience with this but I've been using it for months now. I was loving
// Vue after using it for a couple of weeks.

// UIKit isn't terrible, it's just not nearly as pleasant to use as Vue or the
// web in general.

import UIKit

protocol PickerDelegate: AnyObject {
    func didUpdate(value: UInt8, id: Int)
}

class PickerVC: UIViewController, UIPickerViewDataSource, UIPickerViewDelegate {
    @IBOutlet weak var picker: UIPickerView!
    
    private var value: UInt8! = nil
    private var cases: [String]! = nil
    private var id: Int! = nil
    private weak var delegate: PickerDelegate? = nil
    
    // --- PickerVC --- //
    
    func initialize<E: Enum>(value: E, id: Int, name: String) {
        self.value = value.rawValue
        self.id = id
        title = name
        
        if cases == nil {
            cases = []
        }
        cases.removeAll()
        cases.reserveCapacity(E.allCases.count)
        for e in E.allCases {
            cases.append(e.description)
        }
    }
    
    func setDelegate(_ delegate: PickerDelegate) {
        self.delegate = delegate
    }
    
    // --- UIViewController --- //
    
    override func viewDidLoad() {
        super.viewDidLoad()
        picker.dataSource = self
        picker.delegate = self
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        if isMovingFromParent {
            delegate?.didUpdate(value: value, id: id)
        }
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        picker.selectRow(Int(value), inComponent: 0, animated: false)
    }
    
    // --- UIPickerViewDataSource --- //
    
    func numberOfComponents(in pickerView: UIPickerView) -> Int {
        1
    }
    
    func pickerView(_ pickerView: UIPickerView, numberOfRowsInComponent component: Int) -> Int {
        cases.count
    }
    
    // --- UIPickerViewDelegate --- //
    
    func pickerView(_ pickerView: UIPickerView, titleForRow row: Int, forComponent component: Int) -> String? {
        cases[row]
    }
    
    func pickerView(_ pickerView: UIPickerView, didSelectRow row: Int, inComponent component: Int) {
        value = UInt8(row)
    }
}
