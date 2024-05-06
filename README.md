I study how to develop operating system in Rust,   
following the    
[Philipp Oppermann's "Writing an OS in Rust" series of blogs ](https://os.phil-opp.com/).     
The Rust 2021 edition is used ( not 2018 edition as in the blog series). 

Branches correspond to the blogs in the series. 

Just finished the [VGA Text Mode](https://os.phil-opp.com/vga-text-mode/) from the series.    
[The VGA Text Mode](https://en.wikipedia.org/wiki/VGA_text_mode) is simple way to print text to the screen using VGA buffer.  

It is possible to run the code using usual:   
`cargo run`   
because the code is configured to use [QEMU](https://www.qemu.org/)  virtual machine,   
so we boot disk image in the virtual machine and can see the result in 
QEMU terminal.  

I faced some problems with `rust_azalyzer` (in vscode), find solutions at the links:    
https://github.com/rust-lang/rust-analyzer/issues/4490     
https://stackoverflow.com/questions/65722396/how-to-avoid-e0463-cant-find-crate-for-test-cant-find-crate-when-building


[Philipp Oppermann's "Writing an OS in Rust" MIT license](https://github.com/phil-opp/blog_os/blob/main/LICENSE-MIT)     

>Copyright (c) 2019 Philipp Oppermann

>Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions ...    



