<!-- https://docs.microsoft.com/en-us/windows/console/console-functions -->

<style>.content { max-width: none; }</style>

# Console Functions

| C/C++/Winapi                                                                                      | Rust |
| ------------------------------------------------------------------------------------------------- | ---- |
| <code>let green : [WORD] = [FOREGROUND_GREEN](winapi::um::wincon::FOREGROUND_GREEN);</code>       | <code>let green = [Attributes]::[FOREGROUND_GREEN](Attributes::FOREGROUND_GREEN);</code>
| <code>let mut out_n : [DWORD] = 0;</code>                                                         | <code>let mut out_n : [DWORD];</code>
| <code>let mut codepage : [UINT] = 437;</code>                                                     | <code>let mut codepage : [CodePage] = [CodePage::IBM437];</code>
| |
| <pre>[AddConsoleAlias]\(<br>    [wchz!]\(`"alias"`).as_ptr() as *mut _,<br>    [wchz!]\(`"value"`).as_ptr() as *mut _,<br>    [wchz!]\(`"cmd.exe"`).as_ptr() as *mut _<br>);</pre> | <pre>[add_console_alias]`("alias", "value", "cmd.exe")?;`</pre>
| <pre>[AddConsoleAlias]\(<br>    [wchz!]\(`"alias"`).as_ptr() as *mut _,<br>    NULL,                                  <br>    [wchz!]\(`"cmd.exe"`).as_ptr() as *mut _<br>);</pre> | <pre>[clear_console_alias]`("alias", (), "cmd.exe")?;`</pre>
| <code>[AllocConsole]\();</code>                                                                   | <code>[alloc_console]\()?;</code>
| <code>[AttachConsole]\(42);</code>                                                                | <code>[attach_console]\(42)?;</code>
| <code>[AttachConsole]\(ATTACH_PARENT_PROCESS);</code>                                             | <code>[attach_console]\(ATTACH_PARENT_PROCESS)?;</code> <br> <code>[attach_console_parent_process]\()?;</code>
| <code>[ClosePseudoConsole]\(...);</code>                                                          | ...
| <code>[CreatePseudoConsole]\(...);</code>                                                         | ...
| <code>[CreateConsoleScreenBuffer]\(...);</code>                                                   | <code>[ConsoleScreenBuffer]::[new](ConsoleScreenBuffer::new)()?;</code>
| <code>[FillConsoleOutputAttribute]\(stdout, green, 1, COORD { X: 2, Y: 3 }, &mut out_n);</code>   | <code>out_n = [fill_console_output_attribute]\(&mut [stdout]\(), green, 1, (2,3))?;</code>
| <code>[FillConsoleOutputCharacter]\(stdout, `'!'`, 1, COORD { X: 2, Y: 3 }, &mut out_n);</code>     | <code>out_n = [fill_console_output_character]\(&mut [stdout]\(), `'!'`, 1, (2,3))?;</code>
| <code>[FlushConsoleInputBuffer]\(stdin);</code>                                                   | <code>[flush_console_input_buffer]\(&mut stdin())?;</code>
| <code>[FreeConsole]\();</code>                                                                    | <code>[free_console]\()?;</code>
| <code>[GenerateConsoleCtrlEvent]\(...);</code>                                                    | ...
| <code>[GetConsoleAlias]\(...);</code>                                                             | <code>[get_console_alias]\(...)?;<br>[get_console_alias_os]\(...)?;</code>
| <code>[GetConsoleAliases]\(...);</code>                                                           | <code>[get_console_aliases]\(...)?;<br>[get_console_aliases_os]\(...)?;</code>
| <code>[GetConsoleAliasesLength]\(...);</code>                                                     | <code>[get_console_aliases_length]\(...)?;</code>
| <code>[GetConsoleAliasExes]\(...);</code>                                                         | <code>[get_console_alias_exes]\(...)?;<br>[get_console_alias_exes_os]\()?;</code>
| <code>[GetConsoleAliasExesLength]\();</code>                                                      | <code>[get_console_alias_exes_length]\()?;</code>
| <code>codepage = [GetConsoleCP]\();</code>                                                        | <code>codepage = [get_console_input_cp]\()?;</code>
| <code>[GetConsoleCursorInfo]\(stdout, &mut info);</code>                                          | <code>let info = [get_console_cursor_info]\(&[stdout]\())?;</code>
| <code>[GetConsoleDisplayMode]\(&mut mode);</code>                                                 | <code>let mode = [get_console_display_mode]\()?;</code>
| <code>let c : [COORD] = [GetConsoleFontSize]\(stdout, 42);</code>                                 | <code>let c : [COORD] = [get_console_font_size]\(&[stdout]\(), 42)?;</code>
| <code>[GetConsoleHistoryInfo]\(&mut info);</code>                                                 | <code>let info = [get_console_history_info]\()?;</code>
| <code>[GetConsoleMode]\(handle, &mut mode);</code>                                                | <code>let mode = [get_console_mode]\(handle)?;</code>
| <code>[GetConsoleOriginalTitleW]\(title.as_mut_ptr(); title.len() as _);</code>                   | <code>let title : [OsString] = [get_console_original_title]\()?;</code>
| <code>codepage = [GetConsoleOutputCP]\();</code>                                                  | <code>codepage = [get_console_output_cp]\()?;</code>
| <code>[GetConsoleProcessList]\(...);</code>                                                       | ...
| <code>[GetConsoleScreenBufferInfo]\(stdout, &mut info);</code>                                    | <code>let info = [get_console_screen_buffer_info]\(&[stdout]\())?;</code>
| <code>[GetConsoleScreenBufferInfoEx]\(stdout, &mut info);</code>                                  | <code>let info = [get_console_screen_buffer_info_ex]\(&[stdout]\())?;</code>
| <code>[GetConsoleSelectionInfo]\(&mut info);</code>                                               | <code>let info = [get_console_selection_info]\()?;</code>
| <code>[GetConsoleTitleW]\(title.as_mut_ptr(), title.len() as _);</code>                           | <code>let title : [OsString] = [get_console_title]\()?;</code>
| <code>let hwnd : [HWND] = [GetConsoleWindow]\();</code>                                           | <code>let hwnd : [HWND] = [get_console_window]\()?;</code>
| <code>[GetCurrentConsoleFont]\(stdout, FALSE, &mut info);</code>                                  | <code>let info = [get_current_console_font]\(&[stdout]\(), false)?;</code>
| <code>[GetCurrentConsoleFontEx]\(stdout, FALSE, &mut info);</code>                                | <code>let info = [get_current_console_font_ex]\(&[stdout]\(), false)?;</code>
| <code>let s : [COORD] = [GetLargestConsoleWindowSize]\(stdout);</code>                            | <code>let s : [COORD] = [get_largest_console_window_size]\(&[stdout]\())?;</code>
| <code>let events = [GetNumberOfConsoleInputEvents]\(stdout);</code>                               | <code>let events = [get_number_of_console_input_events]\(&[stdout]\())?;</code>
| <code>let buttons = [GetNumberOfConsoleMouseButtons]\();</code>                                   | <code>let buttons = [get_number_of_console_mouse_buttons]\()?;</code>
| <pre>use winapi::um::{processenv, winbase}::*;<br>let h = [GetStdHandle]\(STD_INPUT_HANDLE);<br>let h = [GetStdHandle]\(STD_OUTPUT_HANDLE);<br>let h = [GetStdHandle]\(STD_ERROR_HANDLE);</pre> | <pre>use [std::os::windows::io::AsRawHandle];<br>let h = [std::io::stdin]\().[as_raw_handle]\().cast();<br>let h = [std::io::stdout]\().[as_raw_handle]\().cast();<br>let h = [std::io::stderr]\().[as_raw_handle]\().cast();</pre>
| <code>[HandlerRoutine]\(...);</code>                                                              | ...
| <pre>let mut out_n = 0;<br>let mut buffer : [INPUT_RECORD; 8] = Default::default();<br>let succeeded : BOOL = [PeekConsoleInput]\(<br>    stdin,<br>    buffer.as_mut_ptr(),<br>    buffer.len() as _,<br>    &mut out_n<br>);<br>if succeeded == 0 { return Err(std::io::Error::last_os_error()); }<br>for record in buffer[0 .. out_n as usize] { ... }</pre> | <pre>for record in [peek_console_input]\(&mut [stdin]\())? { ... }<br><br>// or:<br><br>for record in [peek_console_input_with]\(&mut [stdin]\(), &mut [_; 8]::default())? { ... }<br><br>// or:<br><br>if let Some(record) = [peek_console_input_one]\(&mut [stdin]\())? { ... }</pre>
| <code>[ReadConsole]\(stdin, ...);</code>                                                          | <code>[read_console]\(&mut [stdin]\(), ...)?;</code>
| <pre>let mut out_n = 0;<br>let mut buffer : [INPUT_RECORD; 8] = Default::default();<br>let succeeded : BOOL = [ReadConsoleInput]\(<br>    stdin,<br>    buffer.as_mut_ptr(),<br>    buffer.len() as _,<br>    &mut out_n<br>);<br>if succeeded == 0 { return Err(std::io::Error::last_os_error()); }<br>for record in buffer[0 .. out_n as usize] { ... }</pre> | <pre>for record in [read_console_input]\(&mut [stdin]\())? { ... }<br><br>// or:<br><br>for record in [read_console_input_with]\(&mut [stdin]\(), &mut [_; 8]::default())? { ... }<br><br>// or:<br><br>let record = [read_console_input_one]\(&mut [stdin]\())?;<br>...</pre>
| <code>[ReadConsoleOutput]\(stdout, ...);</code>                                                   | <code>[read_console_output]\(&[stdout]\(), ...)?;</code>
| <code>[ReadConsoleOutputAttribute]\(stdout, ...);</code>                                          | <code>[read_console_output_attribute]\(&[stdout]\(), ...)?;</code>
| <code>[ReadConsoleOutputCharacter]\(stdout, ...);</code>                                          | <code>[read_console_output_character]\(&[stdout]\(), ...)?;</code>
| <code>[ResizePseudoConsole]\(...);</code>                                                         | ...
| <code>[ScrollConsoleScreenBuffer]\(...);</code>                                                   | ...
| <code>[SetConsoleActiveScreenBuffer]\(stdout);</code>                                             | <code>[set_console_active_screen_buffer]\(&[stdout]\())?;</code>
| <code>[SetConsoleCP]\(437);</code>                                                                | <code>[set_console_input_cp]\(437)?; <br> [set_console_input_cp]\([CodePage]::[IBM437](CodePage::IBM437))?; <br> [set_console_input_cp]\([CodePage]::from(437))?; <br> let _s = [InputCodePageScope]::[new](InputCodePageScope::new)([CodePage]::[IBM437](CodePage::IBM437))?;</code>
| <code>[SetConsoleCtrlHandler]\(...);</code>                                                       | ...
| <code>[SetConsoleCursorInfo]\(stdout, ...);</code>                                                | <code>[set_console_cursor_info]\(&mut [stdout]\(), ...)?;</code>
| <code>[SetConsoleCursorPosition]\(stdout, COORD { X: 1, Y: 2 })</code>                            | <code>[set_console_cursor_position]\(&mut [stdout]\(), (1, 2))?;</code>
| <code>[SetConsoleDisplayMode]\(...);</code>                                                       | ...
| <code>[SetConsoleHistoryInfo]\(...);</code>                                                       | ...
| <code>[SetConsoleMode]\(handle, mode);</code>                                                     | <code>[set_console_mode]\(handle, mode)?; <br> [change_console_mode]\(handle, \|_old_mode\| mode)?;</code>
| <code>[SetConsoleOutputCP]\(437);</code>                                                          | <code>[set_console_output_cp]\(437)?; <br> [set_console_output_cp]\([CodePage]::[IBM437](CodePage::IBM437))?; <br> [set_console_output_cp]\([CodePage]::from(437))?; <br> let _s = [OutputCodePageScope]::[new](OutputCodePageScope::new)([CodePage]::[IBM437](CodePage::IBM437))?;</code>
| <code>[SetConsoleScreenBufferInfoEx]\(...);</code>                                                | ...
| <code>[SetConsoleScreenBufferSize]\(...);</code>                                                  | ...
| <code>[SetConsoleTextAttribute]\(stdout, green);</code>                                           | <code>[set_console_text_attribute]\(&mut [stdout]\(), green)?;</code>
| <code>[SetConsoleTitleW]\([wchz!]\(`"new title"`).as_ptr());</code>                               | <code>[set_console_title]\(`"new title"`)?;</code>
| <code>[SetConsoleWindowInfo]\(stdout, FALSE, &[SMALL_RECT] { Left: 0, Top: 0, Right: 80, Bottom: 25 });</code> | <code>[set_console_window_info]\(&mut [stdout]\(), false, (0,0) .. (80,25))?;</code>
| <code>[SetCurrentConsoleFontEx]\(...);</code>                                                     | ...
| <code>[SetStdHandle]\(...);</code>                                                                | ...
| <code>[WriteConsole]\(stdout, ...);</code>                                                        | <code>[write_console]\(&mut [stdout]\(), ...)?;</code>
| <code>[WriteConsoleInput]\(stdin, ...);</code>                                                    | <code>[write_console_input]\(&mut [stdin]\(), ...)?;</code>
| <code>[WriteConsoleOutput]\(stdout, ...);</code>                                                  | <code>[write_console_output]\(&mut [stdout]\(), ...)?;</code>
| <code>[WriteConsoleOutputAttribute]\(stdout, ...);</code>                                         | <code>[write_console_output_attribute]\(&mut [stdout]\(), ...)?;</code>
| <code>[WriteConsoleOutputCharacter]\(stdout, ...);</code>                                         | <code>[write_console_output_character]\(&mut [stdout]\(), ...)?;</code>

[AddConsoleAlias]:                  https://docs.microsoft.com/en-us/windows/console/addconsolealias
[AllocConsole]:                     https://docs.microsoft.com/en-us/windows/console/allocconsole
[AttachConsole]:                    https://docs.microsoft.com/en-us/windows/console/attachconsole
[ClosePseudoConsole]:               https://docs.microsoft.com/en-us/windows/console/closepseudoconsole
[CreatePseudoConsole]:              https://docs.microsoft.com/en-us/windows/console/createpseudoconsole
[CreateConsoleScreenBuffer]:        https://docs.microsoft.com/en-us/windows/console/createconsolescreenbuffer
[FillConsoleOutputAttribute]:       https://docs.microsoft.com/en-us/windows/console/fillconsoleoutputattribute
[FillConsoleOutputCharacter]:       https://docs.microsoft.com/en-us/windows/console/fillconsoleoutputcharacter
[FlushConsoleInputBuffer]:          https://docs.microsoft.com/en-us/windows/console/flushconsoleinputbuffer
[FreeConsole]:                      https://docs.microsoft.com/en-us/windows/console/freeconsole
[GenerateConsoleCtrlEvent]:         https://docs.microsoft.com/en-us/windows/console/generateconsolectrlevent
[GetConsoleAlias]:                  https://docs.microsoft.com/en-us/windows/console/getconsolealias
[GetConsoleAliases]:                https://docs.microsoft.com/en-us/windows/console/getconsolealiases
[GetConsoleAliasesLength]:          https://docs.microsoft.com/en-us/windows/console/getconsolealiaseslength
[GetConsoleAliasExes]:              https://docs.microsoft.com/en-us/windows/console/getconsolealiasexes
[GetConsoleAliasExesLength]:        https://docs.microsoft.com/en-us/windows/console/getconsolealiasexeslength
[GetConsoleCP]:                     https://docs.microsoft.com/en-us/windows/console/getconsolecp
[GetConsoleCursorInfo]:             https://docs.microsoft.com/en-us/windows/console/getconsolecursorinfo
[GetConsoleDisplayMode]:            https://docs.microsoft.com/en-us/windows/console/getconsoledisplaymode
[GetConsoleFontSize]:               https://docs.microsoft.com/en-us/windows/console/getconsolefontsize
[GetConsoleHistoryInfo]:            https://docs.microsoft.com/en-us/windows/console/getconsolehistoryinfo
[GetConsoleMode]:                   https://docs.microsoft.com/en-us/windows/console/getconsolemode
[GetConsoleOriginalTitleW]:         https://docs.microsoft.com/en-us/windows/console/getconsoleoriginaltitle
[GetConsoleOutputCP]:               https://docs.microsoft.com/en-us/windows/console/getconsoleoutputcp
[GetConsoleProcessList]:            https://docs.microsoft.com/en-us/windows/console/getconsoleprocesslist
[GetConsoleScreenBufferInfo]:       https://docs.microsoft.com/en-us/windows/console/getconsolescreenbufferinfo
[GetConsoleScreenBufferInfoEx]:     https://docs.microsoft.com/en-us/windows/console/getconsolescreenbufferinfoex
[GetConsoleSelectionInfo]:          https://docs.microsoft.com/en-us/windows/console/getconsoleselectioninfo
[GetConsoleTitleW]:                 https://docs.microsoft.com/en-us/windows/console/getconsoletitle
[GetConsoleWindow]:                 https://docs.microsoft.com/en-us/windows/console/getconsolewindow
[GetCurrentConsoleFont]:            https://docs.microsoft.com/en-us/windows/console/getcurrentconsolefont
[GetCurrentConsoleFontEx]:          https://docs.microsoft.com/en-us/windows/console/getcurrentconsolefontex
[GetLargestConsoleWindowSize]:      https://docs.microsoft.com/en-us/windows/console/getlargestconsolewindowsize
[GetNumberOfConsoleInputEvents]:    https://docs.microsoft.com/en-us/windows/console/getnumberofconsoleinputevents
[GetNumberOfConsoleMouseButtons]:   https://docs.microsoft.com/en-us/windows/console/getnumberofconsolemousebuttons
[GetStdHandle]:                     https://docs.microsoft.com/en-us/windows/console/getstdhandle
[HandlerRoutine]:                   https://docs.microsoft.com/en-us/windows/console/handlerroutine
[PeekConsoleInput]:                 https://docs.microsoft.com/en-us/windows/console/peekconsoleinput
[ReadConsole]:                      https://docs.microsoft.com/en-us/windows/console/readconsole
[ReadConsoleInput]:                 https://docs.microsoft.com/en-us/windows/console/readconsoleinput
[ReadConsoleOutput]:                https://docs.microsoft.com/en-us/windows/console/readconsoleoutput
[ReadConsoleOutputAttribute]:       https://docs.microsoft.com/en-us/windows/console/readconsoleoutputattribute
[ReadConsoleOutputCharacter]:       https://docs.microsoft.com/en-us/windows/console/readconsoleoutputcharacter
[ResizePseudoConsole]:              https://docs.microsoft.com/en-us/windows/console/resizepseudoconsole
[ScrollConsoleScreenBuffer]:        https://docs.microsoft.com/en-us/windows/console/scrollconsolescreenbuffer
[SetConsoleActiveScreenBuffer]:     https://docs.microsoft.com/en-us/windows/console/setconsoleactivescreenbuffer
[SetConsoleCP]:                     https://docs.microsoft.com/en-us/windows/console/setconsolecp
[SetConsoleCtrlHandler]:            https://docs.microsoft.com/en-us/windows/console/setconsolectrlhandler
[SetConsoleCursorInfo]:             https://docs.microsoft.com/en-us/windows/console/setconsolecursorinfo
[SetConsoleCursorPosition]:         https://docs.microsoft.com/en-us/windows/console/setconsolecursorposition
[SetConsoleDisplayMode]:            https://docs.microsoft.com/en-us/windows/console/setconsoledisplaymode
[SetConsoleHistoryInfo]:            https://docs.microsoft.com/en-us/windows/console/setconsolehistoryinfo
[SetConsoleMode]:                   https://docs.microsoft.com/en-us/windows/console/setconsolemode
[SetConsoleOutputCP]:               https://docs.microsoft.com/en-us/windows/console/setconsoleoutputcp
[SetConsoleScreenBufferInfoEx]:     https://docs.microsoft.com/en-us/windows/console/setconsolescreenbufferinfoex
[SetConsoleScreenBufferSize]:       https://docs.microsoft.com/en-us/windows/console/setconsolescreenbuffersize
[SetConsoleTextAttribute]:          https://docs.microsoft.com/en-us/windows/console/setconsoletextattribute
[SetConsoleTitleW]:                 https://docs.microsoft.com/en-us/windows/console/setconsoletitle
[SetConsoleWindowInfo]:             https://docs.microsoft.com/en-us/windows/console/setconsolewindowinfo
[SetCurrentConsoleFontEx]:          https://docs.microsoft.com/en-us/windows/console/setcurrentconsolefontex
[SetStdHandle]:                     https://docs.microsoft.com/en-us/windows/console/setstdhandle
[WriteConsole]:                     https://docs.microsoft.com/en-us/windows/console/writeconsole
[WriteConsoleInput]:                https://docs.microsoft.com/en-us/windows/console/writeconsoleinput
[WriteConsoleOutput]:               https://docs.microsoft.com/en-us/windows/console/writeconsoleoutput
[WriteConsoleOutputAttribute]:      https://docs.microsoft.com/en-us/windows/console/writeconsoleoutputattribute
[WriteConsoleOutputCharacter]:      https://docs.microsoft.com/en-us/windows/console/writeconsoleoutputcharacter

[COORD]:                                https://docs.microsoft.com/en-us/windows/console/coord-str
[SMALL_RECT]:                           https://docs.microsoft.com/en-us/windows/console/small-rect-str

[SHORT]:                                https://docs.rs/winapi/0.3.9/winapi/shared/minwindef/type.SHORT.html
[WORD]:                                 https://docs.rs/winapi/0.3.9/winapi/shared/minwindef/type.WORD.html
[DWORD]:                                https://docs.rs/winapi/0.3.9/winapi/shared/minwindef/type.DWORD.html
[UINT]:                                 https://docs.rs/winapi/0.3.9/winapi/shared/minwindef/type.UINT.html
[HWND]:                                 https://docs.rs/winapi/0.3.9/winapi/shared/windef/type.HWND.html

[OsString]:                             https://doc.rust-lang.org/std/ffi/struct.OsString.html
[stdin]:                                https://doc.rust-lang.org/std/io/fn.stdin.html
[stdout]:                               https://doc.rust-lang.org/std/io/fn.stdout.html
[as_raw_handle]:                        https://doc.rust-lang.org/std/os/windows/io/trait.AsRawHandle.html#tymethod.as_raw_handle
[std::os::windows::io::AsRawHandle]:    https://doc.rust-lang.org/std/os/windows/io/trait.AsRawHandle.html

[wchz!]:                                https://docs.rs/wchar/0.11.0/wchar/macro.wchz.html

[x]:    https://img.shields.io/badge/impl-???-red
[?]:    https://img.shields.io/badge/impl-%3f-yellow
[o]:    https://img.shields.io/badge/impl-???-green

<!--
[x]:    https://img.shields.io/badge/impl-x-red
[?]:    https://img.shields.io/badge/impl-%3f-yellow
[o]:    https://img.shields.io/badge/impl-o-green

[x]:    https://img.shields.io/badge/impl-missing-red
[o]:    https://img.shields.io/badge/impl-finished-green
-->
