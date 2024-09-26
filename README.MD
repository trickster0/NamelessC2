# Nameless C2 - A C2 with all its components written in Rust.  
Nameless C2 is a small project I started for fun to get a bit familiar with Rust but I do not have any time to continue working on it,  
hence I am publishing it in hope that some might find it useful to build on top of it since it needs a LOT of work.

## Another damn C2? WHY?!  
That is totally fair to think that and I agree, I am mostly publishing it due to the fact that I have not seen another Windows Rust implant as small as 256kb (if my memory serves me right) and also has a working sleeping obfuscation method of EkkoEx. Is this the best C2 or super OPSEC? Definitely not! Is it worth playing around with it? I will leave that up to you.  

## Installation Process  
I highly recommend that you build the terminal and the implant in Windows boxes while the server should be built ideally on debian.  
1) Dependencies on debian for the NamelessServer:  
  - sudo apt install libssl-dev libsqlite3-dev.
  - goes without saying... install rust  
  
2) Compile terminal and agent ideally with the below command. 
  - cargo build --release --target x86_64-pc-windows-gnu. 

## Notes
There are some stuff left around to take a look and discover, I am not gonna go into how to build things, don't bother opening issues, I am not gonna fix them.  
- The terminal application works better in Windows than Linux.  
- The implant generates a DLL with an export called DiagnosisCheck, to generate shellcode I have provided a modified sRDI which you can simply use by running  
python3 ConvertToShellcode.py -c -f DiagnosisCheck NamelessImplant.dll  
- Make sure you check in the implant the kill date, change the hardcoded IPs/Domains. 
- Coming form PoshC2, this C2 has a similar way of obtaining the output. When the server will start,  
it will generate a NamelessLog.txt which you can tail command to obtain and keep up with everything  
tail -f NamelessLog.txt  
- For the Rust gurus, you will notice stupid choices in the code that make absolutely no sense and I agree but I suck at Rust ¯\_(ツ)_/¯  

## Credits
- thanks to https://x.com/_yamakadi for his amazing work on the Clroxide that executes assemblies.
- thanks to https://x.com/_Kudaes_ for providing his sRDI, you are a lifesaver dude.
- Thanks to https://x.com/memN0ps for his weird(you know what I am talking about) indirect syscall.
- Thanks to https://x.com/lefterispan & https://x.com/eks_perience because I would not be who I am today.  
