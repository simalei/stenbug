# stenbug
Simple wallpaper utility for Windows based on Wallhaven API
# How to use
* Download `stenbug.exe` from `Releases` page or build it yourself from source code
* (Optional) Download `add-to-startup.bat` file to set Stenbug to launch automatically after computer start
* (Optional) Run `add-to-startup.bat` file and input path of `stenbug.exe`. That's it! 
* Start the program
* Stenbug will appear in the tray, where you can open config, change wallpaper, or quit.
# Configuration
Configuration is kept at `%AppData%\stenbug.toml`
|               key |                                                  description  |
|-------------------|---------------------------------------------------------------|
| `system.duration`  | Change picture every...                                      |
| `search.query`     | Search query for Wallhaven (e.g. mountains, landscapes, etc.)|
| `search.categories`| 100/101/**111**/etc (general/anime/people)                   |
| `search.purity`    | **100**/110/111/etc (sfw/sketchy/nsfw)                       |
| `search.sorting`   | **date_added**, relevance, random, views, favorites, toplist |
| `search.order`     | **desc**, asc                                                |
| `search.top_range` | 1d, 3d, 1w,**1M**, 3M, 6M, 1y                                |
| `search.at_least`  | Minimum resolution allowed                                   |
| `search.resolutions` | List of exact wallpaper resolutions (Single resolution allowed) |
| `search.ratios`    | List of aspect ratios |
| `search.colors`    | Search by color |
| `search.api_key`   | Wallhaven API key, required for nsfw |

**All fields are mandatory, however, can be left blank for default value**

Documentation for `search` fields is almost an exact copy of one by [Wallhaven](https://wallhaven.cc/help/api), since all of these fields are passed to the Wallhaven at the time of request. 

# Caveats
Due to borrow-checker constrains that I haven't been able to solve (yet), change in duration requires full restart of the app. This doesn't impact other settings.

# Gallery
![image](https://github.com/user-attachments/assets/cb889e6b-1b2f-4bf8-a67e-32ae22a9a7c3)
