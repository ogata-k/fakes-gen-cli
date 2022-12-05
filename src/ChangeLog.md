# v0.2.6
* update rand crate for security
* update chrono crate
* cleanup code

# v0.2.4
* update regex crate for security

# v0.2.2
* Fix bug: failed parse fake option: With.Join(asd#-#Name.FullName()#Fixed.String(hoge)#4#Fixed.NotString(34)) because of not accept empty value
* Modify: get empty value from empty list.    

# v0.2.1
* modify error message

# v0.2.0
* Move all format action into "file_converter"
* Remove empty value filtering
* Add "With.Join" fake-option. <br/>
e.g. fakes-gen With.Join(_#_dd_#Select.String(hoge#sss)#2#Select.NotString(1#2#3))  // "sss_dd_1_dd_2"
* Modify Json key of full-form format from now as date-time to dummy.
* Modify example cas of csv
* Modify ReadMe

# v0.1.2
* Change --header flag to --fullform and modify behavior when using this flag.
* Modify message of the command ```fakes-gen --bnf```.
* Modify furigana of last name.
* Add cli section to README.md

# v0.1.1
* Modify lacked information.

# v0.1.0
* First publish on crates.io. But, found many lack information in Cargo.toml. So yanked.