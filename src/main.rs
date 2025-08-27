use std::{io::{self, Stdout}, thread, time::{Duration, Instant}};

use crossterm::{cursor::{self, MoveTo, RestorePosition, SavePosition, SetCursorStyle}, event::{self, Event, KeyCode}, style::{Color, Print, SetForegroundColor}, terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize}};

macro_rules! execute {
    ($stdout:expr $(, $command:expr)* $(,)?) => {
        {
            let mut lock = $stdout.lock();
            crossterm::execute!(lock $(, $command)*)
        }
    };
}

enum Lyric {
    Line(Line),
    PartialLine(Line),
    Clear(u64),
    Ascii(usize, u64)
}
#[derive(Clone, Copy)]
struct Line {
    text: &'static str,
    start: u64,
    end: u64
}
const LYRICS: &[Lyric] = &[
    Lyric::Line(Line {
        text: "Forms FORM-29827281-12.",
        start: 0,
        end: 1933,
    }),
    Lyric::Line(Line {
        text: "Test Assessment Report",
        start: 2000,
        end: 3969,
    }),
    Lyric::Line(Line {
        text: "",
        start: 3969,
        end: 3969
    }),
    Lyric::Line(Line {
        text: "",
        start: 3969,
        end: 3969
    }),
    Lyric::Line(Line {
        text: "This was a triumph.",
        start: 6900,
        end: 8866,
    }),
    Lyric::Line(Line {
        text: "I'm making a note here:",
        start: 10833,
        end: 12866
    }),
    Lyric::Line(Line {
        text: "HUGE SUCCESS.",
        start: 13066,
        end: 14766
    }),
    Lyric::Line(Line {
        text: "It's hard to overstate",
        start: 15833,
        end: 18133
    }),
    Lyric::Line(Line {
        text: "my satisfaction.",
        start: 18333,
        end: 20866
    }),
    Lyric::Ascii(0, 23050),
    Lyric::Line(Line {
        text: "Aperture Science",
        start: 23100,
        end: 24766
    }),
    Lyric::Line(Line {
        text: "We do what we must",
        start: 26933,
        end: 28433
    }),
    Lyric::Line(Line {
        text: "because we can.",
        start: 28733,
        end: 30200
    }),
    Lyric::Line(Line {
        text: "For the good of all of us.",
        start: 32000,
        end: 34933
    }),
    Lyric::Ascii(1, 35220),
    Lyric::Line(Line {
        text: "Except the ones who are dead.",
        start: 35266,
        end: 36966
    }),
    Lyric::Line(Line {
        text: "",
        start: 37400,
        end: 37400
    }),
    Lyric::Ascii(0, 37350),
    Lyric::Line(Line {
        text: "But there's no sense crying",
        start: 37400,
        end: 39233
    }),
    Lyric::Line(Line {
        text: "over every mistake.",
        start: 39300,
        end: 41000
    }),
    Lyric::Line(Line {
        text: "You just keep on trying",
        start: 41300,
        end: 43200
    }),
    Lyric::Line(Line {
        text: "till you run out of cake.",
        start: 43266,
        end: 44966
    }),
    Lyric::Ascii(2, 45150),
    Lyric::Line(Line {
        text: "And the science gets done.",
        start: 45200,
        end: 47133
    }),
    Lyric::Line(Line {
        text: "And you make a neat gun.",
        start: 47200,
        end: 49100
    }),
    Lyric::Ascii(0, 49100),
    Lyric::Line(Line {
        text: "For the people who are",
        start: 49166,
        end: 50633
    }),
    Lyric::Line(Line {
        text: "still alive.",
        start: 50766,
        end: 52200
    }),
    Lyric::Clear(53666),
    Lyric::Line(Line {
        text: "Form FORMS-55551-5:",
        start: 53766,
        end: 54333
    }),
    Lyric::Line(Line {
        text: "Personnel File Addendum",
        start: 54366,
        end: 55400
    }),
    Lyric::Line(Line {
        text: "",
        start: 56100,
        end: 56100
    }),
    Lyric::Line(Line {
        text: "Dear <<Subject Name Here>>,",
        start: 56133,
        end: 58400
    }),
    Lyric::Line(Line {
        text: "",
        start: 58400,
        end: 58400
    }),
    Lyric::Line(Line {
        text: "I'm not even angry.",
        start: 58566,
        end: 60400
    }),
    Lyric::PartialLine(Line {
        text: "I'm being ",
        start: 60000 + 2800,
        end: 60000 + 3766
    }),
    Lyric::PartialLine(Line {
        text: "so",
        start: 60000 + 3766,
        end: 60000 + 4166
    }),
    Lyric::Line(Line {
        text: " since right now.",
        start: 60000 + 4166,
        end: 60000 + 6133
    }),
    Lyric::Ascii(3, 69733),
    Lyric::Line(Line {
        text: "Even though you broke my heart.",
        start: 67866,
        end: 71233
    }),
    Lyric::Line(Line {
        text: "And killed me.",
        start: 71300,
        end: 72666
    }),
    Lyric::Ascii(6, 74666),
    Lyric::Line(Line {
        text: "And tore me to pieces.",
        start: 74666,
        end: 76266
    }),
    Lyric::Ascii(4, 81300),
    Lyric::Line(Line {
        text: "And threw every piece into a fire.",
        start: 78633,
        end: 82166
    }),
    Lyric::Line(Line {
        text: "As they burned it hurt because",
        start: 83900,
        end: 86733
    }),
    Lyric::Ascii(5, 87100),
    Lyric::Line(Line {
        text: "I was so happy for you!",
        start: 87133,
        end: 88700
    }),
    Lyric::Line(Line {
        text: "Now these points of data",
        start: 89200,
        end: 60000 + 31200
    }),
    Lyric::Line(Line {
        text: "make a beautiful line.",
        start: 60000 + 31266,
        end: 60000 + 33133
    }),    
    Lyric::Line(Line {
        text: "And we're out of beta.",
        start: 60000 + 33200,
        end: 60000 + 35200
    }),
    Lyric::Line(Line {
        text: "We're releasing on time.",
        start: 60000 + 35233,
        end: 60000 + 37100
    }),
    Lyric::Ascii(6, 60000 + 37100),
    Lyric::Line(Line {
        text: "So I'm GLaD. I got burned.",
        start: 60000 + 37133,
        end: 60000 + 39200
    }),
    Lyric::Ascii(2, 60000 + 39200),
    Lyric::Line(Line {
        text: "Think of all the things we learned",
        start: 60000 + 39300,
        end: 60000 + 41266
    }),
    Lyric::Ascii(0, 60000 + 41250),
    Lyric::Line(Line {
        text: "for the people who are",
        start: 60000 + 41300,
        end: 60000 + 42800
    }),
    Lyric::Line(Line {
        text: "still alive.",
        start: 60000 + 42866,
        end: 60000 + 44433
    }),
    Lyric::Clear(60000 + 46100),
    Lyric::Line(Line {
        text: "Forms FORM-55551-6:",
        start: 60000 + 46133,
        end: 60000 + 46666
    }),
    Lyric::Line(Line {
        text: "Personnel File Addendum Addendum:",
        start: 60000 + 46700,
        end: 60000 + 48000
    }),
    Lyric::Line(Line {
        text: "",
        start: 60000 + 48200,
        end: 60000 + 48200
    }),
    Lyric::Line(Line {
        text: "One more thing:",
        start: 60000 + 48200,
        end: 60000 + 50866
    }),
    Lyric::Line(Line {
        text: "",
        start: 60000 + 50966,
        end: 60000 + 50966
    }),
    Lyric::Line(Line {
        text: "Go ahead and leave me.",
        start: 60000 + 50966,
        end: 60000 + 52866
    }),
    Lyric::Line(Line {
        text: "I think I prefer to stay inside.",
        start: 60000 + 54700,
        end: 60000 + 58500
    }),
    Lyric::Line(Line {
        text: "Maybe you'll find someone else",
        start: 120000 + 100,
        end: 120000 + 3433,
    }),
    Lyric::Line(Line {
        text: "to help you.",
        start: 120000 + 3500,
        end: 120000 + 4666,
    }),
    Lyric::Ascii(7, 120000 + 7400),
    Lyric::PartialLine(Line {
        text: "Maybe Black ",
        start: 120000 + 6966,
        end: 120000 + 8000,
    }),
    Lyric::Line(Line {
        text: "Mesa...",
        start: 120000 + 8000,
        end: 120000 + 10400,
    }),
    Lyric::PartialLine(Line {
        text: "THAT WAS A JOKE.",
        start: 120000 + 11000,
        end: 120000 + 12333,
    }),
    Lyric::Line(Line {
        text: " FAT CHANCE.",
        start: 120000 + 13400,
        end: 120000 + 14566,
    }),
    Lyric::Ascii(8, 120000 + 17400),
    Lyric::PartialLine(Line {
        text: "Anyway,",
        start: 120000 + 16166,
        end: 120000 + 17500,
    }),
    Lyric::Line(Line {
        text: " this cake is great.",
        start: 120000 + 17500,
        end: 120000 + 19200,
    }),
    Lyric::Line(Line {
        text: "It's so delicious and moist.",
        start: 120000 + 19233,
        end: 120000 + 21100,
    }),
    Lyric::Ascii(9, 120000 + 21550),
    Lyric::Line(Line {
        text: "Look at me still talking",
        start: 120000 + 21566,
        end: 120000 + 23433,
    }),
    Lyric::Ascii(1, 120000 + 23433),
    Lyric::Line(Line {
        text: "when there's Science to do.",
        start: 120000 + 23500,
        end: 120000 + 25533,
    }),
    Lyric::Ascii(0, 120000 + 25533),
        Lyric::Line(Line {
        text: "When I look out there,",
        start: 120000 + 25600,
        end: 120000 + 27233,
    }),
        Lyric::Line(Line {
        text: "it makes me GLaD I'm not you.",
        start: 120000 + 27300,
        end: 120000 + 29400,
    }),
    Lyric::Ascii(2, 120000 + 29450),
    Lyric::Line(Line {
        text: "I've experiments to run.",
        start: 120000 + 29466,
        end: 120000 + 31466,
    }),
    Lyric::Ascii(6, 120000 + 31466),
    Lyric::Line(Line {
        text: "There is research to be done.",
        start: 120000 + 31500,
        end: 120000 + 33466,
    }),
    Lyric::Ascii(0, 120000 + 33466),
    Lyric::Line(Line {
        text: "On the people who are",
        start: 120000 + 33533,
        end: 120000 + 35100,
    }),
    Lyric::PartialLine(Line {
        text: "still ",
        start: 120000 + 35133,
        end: 120000 + 35433,
    }),
    Lyric::Line(Line {
        text: "alive.",
        start: 120000 + 35633,
        end: 120000 + 36900,
    }),
];

// https://blog.kazitor.com/2014/12/portal-ascii/ - awesome work, thats some serious effort
const ASCII_ART: &[&[&'static str]] = &[
&[
"              .,-:;//;:=,",
"          . :H@@@MM@M#H/.,+%;,",
"       ,/X+ +M@@M@MM%=,-%HMMM@X/,",
"     -+@MM; $M@@MH+-,;XMMMM@MMMM@+-",
"    ;@M@@M- XM@X;. -+XXXXXHHH@M@M#@/.",
"  ,%MM@@MH ,@%=            .---=-=:=,.",
"  -@#@@@MX .,              -%HX$$%%%+;",
" =-./@M@M$                  .;@MMMM@MM:",
" X@/ -$MM/                    .+MM@@@M$",
",@M@H: :@:                    . -X#@@@@-",
",@@@MMX, .                    /H- ;@M@M=",
".H@@@@M@+,                    %MM+..%#$.",
" /MMMM@MMH/.                  XM@MH; -;",
"  /%+%$XHH@$=              , .H@@@@MX,",
"   .=--------.           -%H.,@@@@@MX,",
"   .%MM@@@HHHXX$$$%+- .:$MMX -M@@MM%.",
"     =XMMM@MM@MM#H;,-+HMM@M+ /MMMX=",
"       =%@M@M#@$-.=$@MM@@@M; %M%=",
"         ,:+$+-,/H#MMMMMMM@- -,",
"               =++%%%%+/:-.",
],
&[
"             =+$HM####@H%;,",
"          /H###############M$,",
"          ,@################+",
"           .H##############+",
"             X############/",
"              $##########/",
"               %########/",
"                /X/;;+X/",
"                 -XHHX-",
"                ,######,",
"#############X  .M####M.  X#############",
"##############-   -//-   -##############",
"X##############%,      ,+##############X",
"-##############X        X##############-",
" %############%          %############%",
"  %##########;            ;##########%",
"   ;#######M=              =M#######;",
"    .+M###@,                ,@###M+.",
"       :XH.                  .HX:",
],
&[
"                 =/;;/-",
"                +:    //",
"               /;      /;",
"              -X        H.",
".//;;;:;;-,   X=        :+   .-;:=;:;%;.",
"M-       ,=;;;#:,      ,:#;;:=,       ,@",
":%           :%.=/++++/=.$=           %=",
" ,%;         %/:+/;,,/++:+/         ;+.",
"   ,+/.    ,;@+,        ,%H;,    ,/+,",
"      ;+;;/= @.  .H##X   -X :///+;",
"      ;+=;;;.@,  .XM@$.  =X.//;=%/.",
"   ,;:      :@%=        =$H:     .+%-",
" ,%=         %;-///==///-//         =%,",
";+           :%-;;;;;;;;-X-           +:",
"@-      .-;;;;M-        =M/;;;-.      -X",
" :;;::;;-.    %-        :+    ,-;;-;:==",
"              ,X        H.",
"               ;/      %=",
"                //    +;",
"                 ,////,",
],
&[
"                          .,---.",
"                        ,/XM#MMMX;,",
"                      -%##########M%,",
"                     -@######%  $###@=",
"      .,--,         -H#######$   $###M:",
"   ,;$M###MMX;     .;##########$;HM###X=",
",/@###########H=      ;################+",
"-+#############M/,      %##############+",
"%M###############=      /##############:",
"H################      .M#############;.",
"@###############M      ,@###########M:.",
"X################,      -$=X#######@:",
"/@##################%-     +######$-",
".;##################X     .X#####+,",
" .;H################/     -X####+.",
"   ,;X##############,       .MM/",
"      ,:+$H@M#######M#$-    .$$=",
"           .,-=;+$@###X:    ;/=.",
"                  .,/X$;   .::,",
"                      .,    ..",
],
&[
"                     -$-",
"                    .H##H,",
"                   +######+",
"                .+#########H.",
"              -$############@.",
"            =H###############@  -X:",
"          .$##################:  @#@-",
"     ,;  .M###################;  H###;",
"   ;@#:  @###################@  ,#####:",
" -M###.  M#################@.  ;######H",
" M####-  +###############$   =@#######X",
" H####$   -M###########+   :#########M,",
"  /####X-   =########%   :M########@/.",
"    ,;%H@X;   .$###X   :##MM@%+;:-",
"                 ..",
"  -/;:-,.              ,,-==+M########H",
" -##################@HX%%+%%$%%%+:,,",
"    .-/H%%%+%%$H@###############M@+=:/+:",
"/XHX%:#####MH%=    ,---:;;;;/&&XHM,:###$",
"$@#MX %+;-                           .",
],
&[
"                                     :X-",
"                                  :X###",
"                                ;@####@",
"                              ;M######X",
"                            -@########$",
"                          .$##########@",
"                         =M############-",
"                        +##############$",
"                      .H############$=.",
"         ,/:         ,M##########M;.",
"      -+@###;       =##########M;",
"   =%M#######;     :#########M/",
"-$M###########;   :########/",
" ,;X###########; =#######$.",
"     ;H#########+######M=",
"       ,+#############+",
"          /M########@-",
"            ;M#####%",
"              +####:",
"               ,$M-",
],
&[
"            .+",
"             /M;",
"              H#@:              ;,",
"              -###H-          -@/",
"               %####$.  -;  .%#X",
"                M#####+;#H :M#M.",
"..          .+/;%#############-",
" -/%H%+;-,    +##############/",
"    .:$M###MH$%+############X  ,--=;-",
"        -/H#####################H+=.",
"           .+#################X.",
"         =%M####################H;.",
"            /@###############+;;/%%;,",
"         -%###################$",
"       ;H######################M=",
"    ,%#####MH$%;+#####M###-/@####%",
"  :$H%+;=-      -####X.,H#   -+M##@-",
" .              ,###;    ;      =$##+",
"                .#H,               :XH,",
"                 +                   .;-",
],
&[
"           .-;+$XHHHHHHX$+;-.",
"        ,;X@@X%/;=----=:/%X@@X/,",
"      =$@@%=.              .=+H@X:",
"    -XMX:                      =XMX=",
"   /@@:                          =H@+",
"  %@X,                            .$@$",
" +@X.                               $@%",
"-@@,                                .@@=",
"%@%                                  +@$",
"H@:                                  :@H",
"H@:         :HHHHHHHHHHHHHHHHHHX,    =@H",
"%@%         ;@M@@@@@@@@@@@@@@@@@H-   +@$",
"=@@,        :@@@@@@@@@@@@@@@@@@@@@= .@@:",
" +@X        :@@@@@@@@@@@@@@@M@@@@@@:%@%",
"  $@$,      ;@@@@@@@@@@@@@@@@@M@@@@@@$.",
"   +@@HHHHHHH@@@@@@@@@@@@@@@@@@@@@@@+",
"    =X@@@@@@@@@@@@@@@@@@@@@@@@@@@@X=",
"      :$@@@@@@@@@@@@@@@@@@@M@@@@$:",
"        ,;$@@@@@@@@@@@@@@@@@@X/-",
"           .-;+$XXHHHHHX$+;-.",
],
&[
"            ,:/+/-",
"            /M/              .,-=;//;-",
"       .:/= ;MH/,    ,=/+%$XH@MM#@:",
"      -$##@+$###@H@MMM#######H:.    -/H#",
" .,H@H@ X######@ -H#####@+-     -+H###@X",
"  .,@##H;      +XM##M/,     =%@###@X;-",
"X%-  :M##########$.    .:%M###@%:",
"M##H,   +H@@@$/-.  ,;$M###@%,          -",
"M####M=,,---,.-%%H####M$:          ,+@##",
"@##################@/.         :%H##@$-",
"M###############H,         ;HM##M$=",
"#################.    .=$M##M$=",
"################H..;XM##M$=          .:+",
"M###################@%=           =+@MH%",
"@#################M/.         =+H#X%=",
"=+M###############M,      ,/X#H+:,",
"  .;XM###########H=   ,/X#H+:;",
"     .=+HM#######M+/+HM@+=.",
"         ,:/%XM####H/.",
"              ,.:=-.",
],
&[
"       #+ @      # #              M#@",
" .    .X  X.%##@;# #   +@#######X. @H%",
"   ,==.   ,######M+  -#####%M####M-    #",
"  :H##M%:=##+ .M##M,;#####/+#######% ,M#",
" .M########=  =@#@.=#####M=M#######=  X#",
" :@@MMM##M.  -##M.,#######M#######. =  M",
"             @##..###:.    .H####. @@ X,",
"   ############: ###,/####;  /##= @#. M",
"           ,M## ;##,@#M;/M#M  @# X#% X#",
".%=   ######M## ##.M#:   ./#M ,M #M ,#$",
"##/         $## #+;#: #### ;#/ M M- @# :",
"#+ #M@MM###M-;M #:$#-##$H# .#X @ + $#. #",
"      ######/.: #%=# M#:MM./#.-#  @#: H#",
"+,.=   @###: /@ %#,@  ##@X #,-#@.##% .@#",
"#####+;/##/ @##  @#,+       /#M    . X,",
"   ;###M#@ M###H .#M-     ,##M  ;@@; ###",
"   .M#M##H ;####X ,@#######M/ -M###$  -H",
"    .M###%  X####H  .@@MM@;  ;@#M@",
"      H#M    /@####/      ,++.  / ==-,",
"               ,=/:, .+X@MMH@#H  #####$=",
]
];

// got this, and the velocity from https://github.com/SDSkyKlouD/still-alive-web/
const CREDITS: &[&'static str] = &[
    ">LIST PERSONNEL",
	"",
	"",
	"Gautam Babbar",
	"Ted Backman",
	"Kelly Bailey",
	"Jeff Ballinger",
	"Aaron Barber",
	"Jeep Barnett",
	"Jeremy Bennett",
	"Dan Berger",
	"Yahn Bernier",
	"Ken Birdwell",
	"Derrick Birum",
	"Mike Blaszczak",
	"Iestyn Bleasdale-Shepherd",
	"Chris Bokitch",
	"Steve Bond",
	"Matt Boone",
	"Antoine Bourdon",
	"Jamaal Bradley",
	"Jason Brashill",
	"Charlie Brown",
	"Charlie Burgin",
	"Andrew Burke",
	"Augusta Butlin",
	"Julie Caldwell",
	"Dario Casali",
	"Chris Chin",
	"Jess Cliffe",
	"Phil Co",
	"John Cook",
	"Christen Coomer",
	"Greg Coomer",
	"Scott Dalton",
	"Kerry Davis",
	"Jason Deakins",
	"Joe Demers",
	"Ariel Diaz",
	"Quintin Doroquez",
	"Jim Dose",
	"Chris Douglass",
	"Laura Dubuk",
	"Mike Dunkle",
	"Mike Durand",
	"Mike Dussault",
	"Dhabih Eng",
	"Katie Engel",
	"Chet Faliszek",
	"Adrian Finol",
	"Bill Fletcher",
	"Moby Francke",
	"Stephane Gaudette",
	"Kathy Gehrig",
	"Vitaliy Genkin",
	"Paul Graham",
	"Chris Green",
	"Chris Grinstead",
	"John Guthrie",
	"Aaron Halifax",
	"Reagan Halifax",
	"Leslie Hall",
	"Jeff Hameluck",
	"Joe Han",
	"Don Holden",
	"Jason Holtman",
	"Gray Horsfield",
	"Keith Huggins",
	"Jim Hughes",
	"Jon Huisingh",
	"Brian Jacobson",
	"Lars Jensvold",
	"Erik Johnson",
	"Jakob Jungels",
	"Rich Kaethler",
	"Steve Kalning",
	"Aaron Kearly",
	"Iikka Keranen",
	"David Kircher",
	"Eric Kirchmer",
	"Scott Klintworth",
	"Alden Kroll",
	"Marc Laidlaw",
	"Jeff Lane",
	"Tim Larkin",
	"Dan LeFree",
	"Isabelle LeMay",
	"Tom Leonard",
	"Jeff Lind",
	"Doug Lombardi",
	"Bianca Loomis",
	"Richard Lord",
	"Realm Lovejoy",
	"Randy Lundeen",
	"Scott Lynch",
	"Ido Magal",
	"Nick Maggiore",
	"John McCaskey",
	"Patrick McClard",
	"Steve McClure",
	"Hamish McKenzie",
	"Gary McTaggart",
	"Jason Mitchell",
	"Mike Morasky",
	"John Morello II",
	"Bryn Moslow",
	"Arsenio Navarro",
	"Gabe Newell",
	"Milton Ngan",
	"Jake Nicholson",
	"Martin Otten",
	"Nick Papineau",
	"Karen Prell",
	"Bay Raitt",
	"Tristan Reidford",
	"Alfred Reynolds",
	"Matt Rhoten",
	"Garret Rickey",
	"Dave Riller",
	"Elan Ruskin",
	"Matthew Russell",
	"Jason Ruymen",
	"David Sawyer",
	"Marc Scaparro",
	"Wade Schin",
	"Matthew Scott",
	"Aaron Seeler",
	"Jennifer Seeley",
	"Taylor Sherman",
	"Eric Smith",
	"Jeff Sorensen",
	"David Speyrer",
	"Jay Stelly",
	"Jeremy Stone",
	"Eric Strand",
	"Kim Swift",
	"Kelly Thornton",
	"Eric Twelker",
	"Carl Uhlman",
	"Doug Valente",
	"Bill Van Buren",
	"Gabe Van Engel",
	"Alex Vlachos",
	"Robin Walker",
	"Joshua Weier",
	"Andrea Wicklund",
	"Greg Winkler",
	"Erik Wolpaw",
	"Doug Wood",
	"Matt T. Wood",
	"Danika Wright",
	"Matt Wright",
	"Shawn Zabecki",
	"Torsten Zabka ",
	"",
	"",
	"",
	"",
 	"'Still Alive' by:",
	"Jonathan Coulton",
	"",
	"",
	"",
 	"Voices:",
	"Ellen McLain - GlaDOS, Turrets",
	"Mike Patton - THE ANGER SPHERE",
	"",
	"",
	"",
	"Voice Casting:",
	"Shana Landsburg\\Teri Fiddleman",
	"",
	"",
	"",
	"",
	"Voice Recording:",
	"Pure Audio, Seattle, WA",
	"",
	"",
	"",
	"",
	"Voice recording",
	"scheduling and logistics:",
	"Pat Cockburn, Pure Audio",
	"",
	"",
	"",
	"",
	"Translations:",
	"SDL",
	"",
	"",
	"",
	"",
	"Crack Legal Team:",
	"Liam Lavery",
	"Karl Quackenbush",
	"Kristen Boraas",
	"Kevin Rosenfield",
	"Alan Bruggeman",
	"Dennis Tessier",
	"",
	"",
	"",
	"Thanks for the use of their face:",
	"Alesia Glidewell - Chell",
	"",
	"",
	"",
	"Special thanks to everyone at:",
	"Alienware",
	"ATI",
	"Dell",
	"Falcon Northwest",
	"Havok",
	"SOFTIMAGE",
	"and Don Kemmis, SLK Technologies",
	"",
	"",
	"",
	"",
	"",
	"",
	"",
	"",
	"",
	"THANK YOU FOR PARTICIPATING",
	"IN THIS",
	"ENRICHMENT CENTER ACTIVITY!!",
	"",
	"",
	"",
	"",
	"",
	"",
	"",
	""
];

const CREDIT_CHARACTER_VELOCITY_MS: f64 = 68.623562; // no seriously. this is apparently what it is in the game's source code.

fn main() -> anyhow::Result<()> {
    let stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    // gamma correction on my mac, and anti-aliasing + hinting + sub-pixel rendering meant i couldnt get the perfect colour for this. itll have to do
    execute!(stdout, SetForegroundColor(Color::Rgb { r: 215, g: 172, b: 60 }))?;
    execute!(stdout, SetCursorStyle::BlinkingUnderScore)?;

    // and now.... a jungle of commented out old code;
    // a horrible habit of mine

    //execute!(stdout, cursor::Show)?;
    println!("Hello, world!");

    thread::spawn(move || {
        loop {
            let stdout = io::stdout();
            if let Ok(Event::Key(key_event)) = event::read() {
                if key_event.code == KeyCode::Char('q') {
                    execute!(stdout, LeaveAlternateScreen).expect("AA");
                    terminal::disable_raw_mode().unwrap();
                    std::process::exit(0);
                }   
            }
        }
    });
    /*thread::spawn(|| {
        let mut stdout = io::stdout();
        loop {
            execute!(stdout, SetAttribute(if visible { Attribute::Underlined } else { Attribute::NoUnderline }))
        }
    })*/
    
    terminal::enable_raw_mode()?;

    clear(&stdout)?;
    //println!("ashdaslkjhfdlakwjfha")

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("stillalive.mp3")?;
    let decoder = rodio::Decoder::try_from(file)?;
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(6900));
        sink.append(decoder);
        sink.sleep_until_end();
    });
    
    let start = Instant::now();
    thread::spawn(|| {
        let stdout = io::stdout();
        thread::sleep(Duration::from_millis(6900));
        for (i, credit) in CREDITS.iter().enumerate() {
            clear_credits(&stdout).unwrap();
            let mut cline = 15;
            for k in CREDITS[i.saturating_sub(15)..i].iter().rev() {
                execute!(stdout, SavePosition, MoveTo(53, cline), Print(k), RestorePosition).unwrap();
                cline -= 1;
            }
            //execute!(stdout, MoveTo(size.0 / 2 + 1, 16)).unwrap();
            type_credit(credit, CREDIT_CHARACTER_VELOCITY_MS * credit.len() as f64, &stdout).unwrap();
        }
    });
    let mut linen = 3;
    for i in 0..LYRICS.len() {
        match LYRICS[i] {
            Lyric::Line(line) => {
                let target = start + Duration::from_millis(line.start);
                thread::sleep(target - Instant::now());
                // loop {
                //     if start.elapsed().as_millis() >= line.start as u128 { break }
                // }
                type_line(&line, &stdout)?;
                execute!(stdout, MoveTo(2, linen))?;
            },
            Lyric::PartialLine(line) => {
                let target = start + Duration::from_millis(line.start);
                thread::sleep(target - Instant::now());
                // loop {
                //     if start.elapsed().as_millis() >= line.start as u128 { break }
                // }
                type_line(&line, &stdout)?;
                linen -= 1;
                //execute!(stdout, MoveTo(1, linen))?;
            },
            Lyric::Clear(time) => {
                let target = start + Duration::from_millis(time);
                thread::sleep(target - Instant::now());
                /*loop {
                    if start.elapsed().as_millis() >= time { break }
                }*/
                linen = 2;
                clear_text(&stdout)?
            },
            Lyric::Ascii(index, time) => {
                if start.elapsed().as_millis() >= time as u128 {
                    clear_ascii(&stdout)?;
                    for line in 0..ASCII_ART[index].len() {
                        execute!(stdout, SavePosition, MoveTo(51, 18 + line as u16), Print(ASCII_ART[index][line]), RestorePosition)?;
                    }
                    linen -= 1;
                    //execute!(stdout, MoveTo(1, linen))?;
                } else {
                    thread::spawn(move || {
                        let stdout = io::stdout();
                        let target = start + Duration::from_millis(time);
                        thread::sleep(target - Instant::now());
                        clear_ascii(&stdout).unwrap();
                        for line in 0..ASCII_ART[index].len() {
                            execute!(stdout, SavePosition, MoveTo(51, 18 + line as u16), Print(ASCII_ART[index][line]), RestorePosition).unwrap();
                        }
                    });
                    linen -= 1;
                }
            }
        }
        linen += 1;
    }
    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn type_line(line: &Line, stdout: &Stdout) -> io::Result<()> {
    let duration = line.end - line.start;
    if duration == 0 || line.text.len() == 0 {
        return Ok(())
    }
    let dur = duration / (line.text.len() - 1) as u64;
    for i in line.text.chars().take(line.text.len() - 1) {
        execute!(stdout, Print(i))?;
        thread::sleep(Duration::from_millis(dur));
    }
    execute!(stdout, Print(line.text.chars().last().unwrap()))?;
    Ok(())
}
fn type_credit(credit: &str, duration: f64, stdout: &Stdout) -> io::Result<()> {
    if duration == 0. || credit.len() == 0 {
        thread::sleep(Duration::from_millis(CREDIT_CHARACTER_VELOCITY_MS as u64));
        return Ok(());
    }
    let dur = duration / (credit.len() - 1) as f64;
    let mut x = 53;
    for i in credit.chars() {
        execute!(stdout, SavePosition, MoveTo(x, 16), Print(i), RestorePosition)?;
        thread::sleep(Duration::from_millis(dur as u64));
        x += 1;
    }
    Ok(())
}

fn clear(stdout: &Stdout) -> io::Result<()> {
    execute!(stdout, SetSize(100, 40), Clear(ClearType::All), MoveTo(0, 0))?;
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
    execute!(stdout, Print("_".repeat(49)), cursor::MoveRight(2), Print("_".repeat(49)))?;
    execute!(stdout, MoveTo(0, 1), Print(("|".to_owned() + &" ".repeat(47) + "|  |" + &" ".repeat(47) + "|\r\n").repeat(16)))?;
    execute!(stdout, MoveTo(51, 17), Print("_".repeat(49)))?;
    execute!(stdout, MoveTo(0, 17), Print(("|".to_owned() + &" ".repeat(47) + "|\r\n").repeat(20)))?;
    execute!(stdout, MoveTo(0, 37), Print("_".repeat(49)))?;
    execute!(stdout, MoveTo(2, 2))?;

    Ok(())
}

fn clear_text(stdout: &Stdout) -> io::Result<()> {
    for i in 1..37 {
        execute!(stdout, SavePosition, MoveTo(1, i), Print(" ".repeat(47)), RestorePosition)?;
    }
    execute!(stdout, MoveTo(2, 2))?;
    Ok(())
}
fn clear_ascii(stdout: &Stdout) -> io::Result<()> {
    for i in 18..39 {
        execute!(stdout, SavePosition, MoveTo(51, i), Print(" ".repeat(49)), RestorePosition)?;
    }
    Ok(())
}
fn clear_credits(stdout: &Stdout) -> io::Result<()> {
    for i in 1..17 {
        execute!(stdout, SavePosition, MoveTo(52, i), Print(" ".repeat((47) as usize)), RestorePosition)?;
    }
    Ok(())
}