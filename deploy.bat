cargo bootimage
scp .\target\x86_64-minios\debug\bootimage-minios.bin hrsemcc:.
ssh hrsemcc -- truncate bootimage-minios.bin --size=1M
scp hrsemcc:bootimage-minios.bin "C:\Users\stephane.nsakala\VirtualBox VMs\minios\bootimage-minios.bin"
del "C:\Users\stephane.nsakala\VirtualBox VMs\minios.vdi"
VBoxManage convertfromraw "C:\Users\stephane.nsakala\VirtualBox VMs\minios\bootimage-minios.bin" "C:\Users\stephane.nsakala\VirtualBox VMs\minios.vdi" --format VDI --variant Fixed
vboxmanage internalcommands sethduuid "C:\Users\stephane.nsakala\VirtualBox VMs\minios.vdi" 79fd38f7-ffc1-4e80-9a5a-93ada4c71e55
VBoxManage.exe startvm minios