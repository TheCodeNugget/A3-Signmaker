# 🚧 A3-Signmaker
Program to generate town signs for A3 terrains, written in rust

<p align="left">
  <a href="https://www.gnu.org/licenses/gpl-3.0.en.html#license-text">
    <img alt="GitHub" src="https://img.shields.io/github/license/TheCodeNugget/A3-Signmaker?style=for-the-badge&logo=gitbook&logoColor=D9E0EE&labelColor=302D41&color=f38ba8">
  <a href="https://github.com/TheCodeNugget/Signmaker/releases/latest">
	  <img alt="GitHub release (with filter)" src="https://img.shields.io/github/v/release/TheCodeNugget/A3-Signmaker?style=for-the-badge&logo=github&color=F2CDCD&logoColor=D9E0EE&labelColor=302D41">
  <a href="https://github.com/TheCodeNugget/A3-Signmaker/issues">
	  <img alt="Issues" src="https://img.shields.io/github/issues/TheCodeNugget/A3-Signmaker?style=for-the-badge&logo=gitbook&logoColor=D9E0EE&labelColor=302D41&color=B5E8E0"></a>
</p>

## :hammer_and_wrench: Usage
Grab the latest release from 
<img alt="GitHub" src="https://img.shields.io/github/license/TheCodeNugget/A3-Signmaker?style=for-the-badge&logo=gitbook&logoColor=D9E0EE&labelColor=302D41&color=f38ba8">
  <a href="https://github.com/TheCodeNugget/Signmaker/releases/latest">
  
Download the latest version and unpack and run with terminal of your choice.

```console
.\A3-Signmaker.exe .\terrainname.hpp 1
```
Where:

`.\terrainname.hpp` is the header file that Terrain Builder creates when exporting a WRP

### :warning: Important
Program uses the the name of `.hpp` file to name the files it generates, TB exports this file with the name of your terrain.

The program will create signs for keypoints with types: `NameCityCapital`, `NameCity` & `nameVillage`

`1` is the selection for the type of sign you'd like created.

### Sign Types
Handle  | Type    
--------|--------------
1       | Altis Sign
2       | Livonia Sign
3       | Malden Sign
4       | Tanoa Sign

## :books: License
Licensed under GNU General Public License ([GPLv3](LICENSE.md))

Any program which depends on or calls for A3-Signmaker functions need not be licensed under the GPLv3 or released under a free software license. Only if it is directly including A3-Signmaker code or redistributing a modified version of A3-Signmaker or its binaries itself would be considered a derivative and therefore be legally required to be released under the terms of the GPLv3.
