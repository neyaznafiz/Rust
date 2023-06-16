/**
 * ------Integer Types in Rust--------

    Length	Signed	Unsigned
     8-bit	  i8    	 u8
     16-bit	  i16   	 u16
     32-bit  	i32   	 u32
     64-bit  	i64   	 u64
     128-bit 	i128  	 u128
     arch	    isize	   usize

     Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

     Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.


*---------------- Integer Literals in Rust-----------------

     Number literals   	  Example
        Decimal	          98_222
        Hex	              0xff
        Octal	            0o77
        Binary	          0b1111_0000
        Byte (u8 only)	  b'A'
*/

/**
 --------------- Numeric Operations --------------

    Rust supports the basic mathematical operations you’d expect for all the number types: addition, subtraction, multiplication, division, and remainder. Integer division truncates toward zero to the nearest integer. The following code shows how you’d use each numeric operation in a let statement:

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
*/

/**
 ---------------- The Boolean Type ----------

    As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool.

    For example:
    let t = true;
    let f: bool = false; // with explicit type annotation
*/


/**
 *--------------- The Character Type --------------

    Rust specify char literals with single quotes, as opposed to string literals which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust. 

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
*/


/**
 ----------------- Compound Types -------------------

    Compound types can group multiple values into one type. 
    Rust has two primitive compound types: tuples and arrays.
*/