#!/bin/bash
JETSON_DIR=~/nvidia/nvidia_sdk/JetPack_5.1_Linux_JETSON_NANO

# line number where payload starts
PAYLOAD_LINE=$(awk '/^__PAYLOAD_BEGINS__/ { print NR + 1; exit 0; }' $0)

#echo "Installing required tools"
#sudo apt install qemu-arm-static -y

#extract tarr'ed files to this directory
tail -n +${PAYLOAD_LINE} $0 | base64 -d | tar -zpvx -C /tmp

if [ ! -d packages ]
then
	echo "Downloading packages:"
	mkdir packages
	cd packages
    wget https://developer.download.nvidia.com/embedded/L4T/r35_Release_v2.1/release/Jetson_Linux_R35.2.1_aarch64.tbz2
	wget https://developer.download.nvidia.com/embedded/L4T/r35_Release_v2.1/release/Tegra_Linux_Sample-Root-Filesystem_R35.2.1_aarch64.tbz2
	cd ..
fi

mkdir -p $JETSON_DIR
cp packages/Jetson_Linux_R35.2.1_aarch64.tbz2 $JETSON_DIR/
pushd $JETSON_DIR
echo "Extracting Jetson Tools"
sudo tar jxf Jetson_Linux_R35.2.1_aarch64.tbz2
rm Jetson_Linux_R35.2.1_aarch64.tbz2
popd
cp packages/Tegra_Linux_Sample-Root-Filesystem_R35.2.1_aarch64.tbz2 $JETSON_DIR/Linux_for_Tegra/rootfs/
cp /tmp/l4t_create_default_user.sh $JETSON_DIR/Linux_for_Tegra/tools/
pushd $JETSON_DIR/Linux_for_Tegra
cd rootfs 
echo "Extracting Sample File System"
sudo tar jxf Tegra_Linux_Sample-Root-Filesystem_R35.2.1_aarch64.tbz2
rm Tegra_Linux_Sample-Root-Filesystem_R35.2.1_aarch64.tbz2
cd ..

echo "Setting the default user:"
cd tools
sudo ./l4t_create_default_user.sh -u tguser -p cc20#12tg -a
cd ..

echo "Setting up networking:"
sudo cp /tmp/01-static-default rootfs/etc/network/interfaces.d
sudo cp ~/70-persistent-net.rules rootfs/etc/udev/rules.d # Update this later.

# echo "Removing unneeded dependencies."
# cd rootfs
# sudo cp /usr/bin/qemu-aarch64-static usr/bin/
# sudo mount --bind /dev dev/
# sudo mount --bind /sys/ sys/
# sudo mount --bind /proc/ proc/
# sudo cp /etc/resolv.conf etc/resolv.conf.host
# sudo mv etc/resolv.conf etc/resolv.conf.saved
# sudo mv etc/resolv.conf.host etc/resolv.conf
# sudo LC_ALL=C LANG=C.UTF-8 chroot . /bin/bash
# source /etc/profile
# sudo apt remove ubuntu-desktop xserver-xorg-core -y
# sudo apt purge libqt.* libgtk.* -y
# sudo apt autoremove -y
# exit
# sudo umount ./proc
# sudo umount ./sys
# sudo umount ./dev
# sudo rm usr/bin/qemu-aarch64-static
# sudo rm etc/resolv.conf
# sudo mv etc/resolv.conf.saved etc/resolv.conf
# sudo rm -rf var/lib/apt/lists/*
# sudo rm -rf dev/*
# sudo rm -rf var/log/*
# sudo rm -rf var/tmp/*
# sudo rm -rf var/cache/apt/archives/*.deb
# sudo rm -rf tmp/*

exit 0

__PAYLOAD_BEGINS__
H4sIADNtM2IAA+1aeXfaVhbPv+hTvCpJY7dILDFOm9SZYizbOgeDhyVpps1QWXqAxlqIljjO8t3n
d58kkGwwSU9mOj2je46N9Ja7vrs9cPaiiRlwI+ITi0+N2IkmccgDNZzf+2pQBzxpteiz8aRVz38C
mvVmo3Wvsdds7O3vt/ZbjXv1Rmu/+fgeq389FjZDHEZGwNi9aEZyb163bf4vCve/qV3YXu3CCOeS
dJ91/MV1YM/mEdsxd1mz3vhRIftUWe+FfqS3Wac/OO8P2iO931NZ23GYWByygEM5b7mlSveBZcAt
O4wC+yKObN9jhmcxKI/ZHgv9ODC5GAFZI7hmUz9wwyq7sqM58wPx6ccRkLi+ZU9t0yAUVWYEnC14
4NpRxC22CPy3toWHaG5E+MeBxnH8K9ubMdP3LJs2hWKTy6OngqmGeoOvkPnTjCHTt7AUZwGSRAYY
JZzGhf+WpjKVeH5km7yKOTsEQoADdIQlT9OzbjAEmqZj2C68SjDSvM0ICObUkTECKa0YzN3BS8IG
MfSlvLBUSMs3Y5d7kdBzgg7bajCFj+mAuQgNgW044UrpwlZib06IRLbHKutxW2ykBZ7hcuKJnle8
z33HwgLPXy0StrCjVK0QIcHrByEYuGYXnA4QhPEZ9yyMcjorYMj1I84SPeEUAquNQ8immMhpJvSn
0RWdhfRwsXDBTTpb2GnTmQvoVHnJ+QrDpTCjU33Ihv3j0cv2QGN4Ph/04QbaETt8hUkN3nD+aqCf
nI7Yab97pA2GrN07wmhvNNAPx6M+BuT2EDtlYKOpdu8V0345H2jDIesPmH523tWBDvgH7d5I14ZV
pvc63fGR3jupMqBgvf6IdfUzfYRlo36VyALX7Y2sf8zOtEHnFK/tQ72rj14Jisf6qEfUjkGuzc7b
g5HeGXfbA3Y+hicPNQbRgPBIH3a6bf1MO1LBAagy7YXWG7HhabvbXSsr8V+Q9FADo+3DLqET1CDr
kT7QOiMSavXUgQbBY7fKhudaR6cH7RcNArUHr6op1qH29zEWYZJ4a5+1TyDhzhbVwDqd8UA7I76h
jeH4cDjSR+ORxk76/SNSOJANtcELvaMNn7Fufyi0Nh5qVdAYtQVxIIHKMI3nw/FQF8rTeyNtMBif
U9jbhfwvoRxC1mlj85HQc78nBIam+oNXhJZ0IcxQZS9PNYwPSLFCY21SxBCa64xyy4APNKHKUU5S
1tNOuvqJ1utoNNsnPC/1obYLu+lDWqAnhF+2QXUsBCdrgS/Cd1w8w1VhVaYfs/bRC51YT5fjJAz1
9NQI1XVOU7WrlBVGwovMwF5ELCkWyNdEtUB+GSTx/HphhKHwaJ+75MNTexYHIq6wkEfxQpLwwRQu
SdPYM8V4HBozvrMrfZAq3Jz7TB7TwFM5e33wISE7oSDxif2qxOwnokivz1/jfcF+IrJXfmCJd4P+
zV8vEVSw4yNTlGxTRWH6lKJnFgREEoHzZ/Jc2chqiDfEKsLNIw8xzzYeqTmMC4ExI5vDyKMEVwB9
+K79njJVumqJd8Y9DqUgVeYwGgKjgYDn+DMEZoFyxeBqIsPCPePCAQ52lLINAyEai8EcXk/gnfth
JCLxl8oe8VlgKPFF7EVxQQPzBC93FsCJIOpFiSVXSyj4kkimyReR4iBXeSExMFxR9lkyK44Mkoyb
JKxc/kJO0BCslli1d4a7cDYfD9g6MReDjXpH//jX2cuzvTsWJ5Ixxdi4BjmSdEDz7+yINaRPucNr
Otzw4sXOLsPxJZ19z6XKIg7nFiEKpuHEsoNPMnvOahZ/W/NiaLf5/NuGJFWQ5pmBpONCfJj1wU7I
37BGffcZ8rFUqbg+GIOOZygCmPImj64WXocy+/ZbFieLVBrYsgUJ0izuoZEtm8BycQ8G7t4CLVXs
KfuVPfgbNMdZnb1+Jg4YxisXiBuXeJja+Bc6HLsbUsXyPdKZv7DyWoKWo8BYZApGftBHOb1nLjRZ
+dkk87MklpC3i6ED+cGOhaXs+4chmA7nRrO1H8YunlHy8v09PMy5YTHFZD/sygXzLgzUGRMjmIUJ
0qu57XBIh0MBoRsw7OvUXCZQZWM2SZtzkF0SXjgHPTx7RrM33F0sobeJeD2IglgsDuf2lM7ccl8x
lIltGTtNYod9/Jj4IT2kLjMY9AdPWQ9Ozz0/ns1JLGyGv4VywhpURegOEiwrws0V4WLE+4qEUxtt
JFwMYF+HMGHbKvGNyJWYSAxN0qGNZvpOLC4wo3tvDce2VpyoTEM8QUWuqqpcOCFpnMmQ8dAwUzfJ
n81w7l9NeOwY6dG04QhXkzmn+vqgubccubKtaH7wQ12qpFxPpjjEB3k/j8OgBr8IeA2tQC2Jn4qI
erXu3miiedZkTKbqpgja8HxOHYMavYvkFeLZ+6+MW529lxEqRUT5hilTMnpeCmF/BKjbsykriYem
Aahgj47hURqc2mmDBGaosUiKmSxZGRkzZKDMLAhfKUugVzwQIHfA6FDkqKKbjAMvv229HMsNCDIL
NKAOTv01Dz1KSTvogW/v2ZVprGD4T/khYXmMKAoyGvq/iL+LEqQKGqsIZ6idZF9F8fxsCHoxOdGO
7AixTk7two6RrjpoSn1qHMeh6OnSG4Fh2lxRinRCvkGE9zdkWFrof1AM2KqQ5efcvJwsAqQb/iZx
N+oqxTUZpZeruW+4NnLH8lgsp8H3NwdMDnw/2ngUUc9Q9ghijxkosmPLp9aWtojTuKwewgjdbSCt
IsXtEwk3uFF5bKC5WoKiMeAm+uxrURdOkdytjSQLlKaFYoFHZk2kB2sjUcJvsCQOknjTcEX8s0mK
iEIXVm+4GyuGEZjz/T0lpNsLc5uKbQ/rUIKJraTbdB+isnlJxet2JiDyMl+Su8O4eaq5XJrEOnnd
ZtLTms13VTU30Cwz2Bo0ueyWL9/XnOvk1ndZhSRHexnaluXIurCGPhrlbtY2tJftiYifVNueHJ09
poZk4RjXzDU86DFQs/JQgUvSgZlZ7uOaKfxRpXYxL0aFrnsUGxYn7C7ZqUskNNH4sIMaC2v/VL/b
MCv4rcnsN8JUWU/sDiKb0ANx/gBsJyCstl1bFr+IZ+i66J6N+p/Qdzhpy72Eh1AvIwigzkfDYKWf
tWS5MuNRdP0z/oZ1le5gEepUS166wK/DZOy1pL3j5jAygugg96jUQvImQ2AptKB5QTFxiYJduTBi
izUarWa9zh7q7LcHH0ba4OxTqobnX8xmbUluqbEvEnnUGf81hC4yelvsNHN+sXc8uss7avdrs0cb
cN2B4/N2izYucJkSTL+G2b8A0zZdrg9zZNf8PVOHhlF/P6U6IEhvR/Lmr7Lz7OpmOSEidzXnwTST
j5Sf1/5XzMW2TCZns0Bpzl3fYk9ardXouj1S2pqT3pJbAab49LWCtZyhrj9t/m/NgcWkx1/OZAVr
QZzaHMVTrZgGV+ey25m0u92DDoomUcGoIsYZFnR4wmQqbqr0FYJfhU/Z+G+5qOxCKCP7Dir1KvL/
BzvZlUw+a35k/gK1XOgkN2tA3ACGyEIXLd9Iz9SYL3CsbhUq4SV3aqq8RSzaji4LRoZKBux3qmDX
VjzZZUgheHxkZhyR7p5S3fJY2fs9kWyLKvOXIFlUuEOpMM06LIqbqvHPUHqu1fmMSHZbOAg2C/x4
cbPYkoFG3rKDljBZSHuLE8e+uBlaHGo3LDeLJn+QrxTLNt6yZTf4u5/e5dbS69vVZS88/0aJJpYH
3KWvAtNyQkVGQ2hk4bXr2N4l3aqmF/GrS/j09l0E2SlbE2KLuLKFdwQbae2d3a2gm93bJIHX8U2U
N6I+DSO0To3mE7WuNtQGK1S0tyP06er+eu3KYkH8/Ja/Z2ysWrSd5Pby+J253A2WPslrt4bUpFKZ
Xb/dYRQ2P19POlwmpdzdMnWOdAMp5MKuOrXCkrMXTSLfdwQGWmJS8N3BS2Ed3Xosrqx0Q7L0Q2Hv
w9p30E/KSjYruEpaL1mK3IWYkqVc15K+JLdyspS7k5waFIxyzQVmi9dhyQppdWtK7P4MLgoNtJTL
x9KNYyL92T+G+D+EeiN1aSUNAv8JGoD9vb0Nv//By/6T5e9/njTp9z+PW5guf//zXwByceb4kj01
TI4HZnvIJo7vLy4M81KSDPrliDL3o4WDFpVH83q6lB6TxWlKqCDVBRyJp/FjU23s/6C26mpTqmCF
a4SXrNlqqdlfvfT0EkoooYQSSiihhBJKKKGEEkoooYQSSiihhBJKKKGEEkoooYQSSiihhBJKKKGE
EkoooYQ/Cv8GcwvUEABQAAA=