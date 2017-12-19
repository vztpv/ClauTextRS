
//!
//! 
//!
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::slice;
use std::str;
use std::collections::VecDeque;

pub struct Utility
{
    //
}

#[inline]
fn isWhitespace(ch : char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n'
}

impl Utility 
{
    //! reserve
    pub fn reserve(inFile : &mut File, result : &mut VecDeque<String>) {
        let mut reader = BufReader::new(inFile);
        let mut data = String::new();

        for line in reader.lines()  {
            let temp = line.unwrap();
            data += &temp;
            data += "\n";
        }

        let mut state = 0;
        let mut token_first = 0;
        let mut token_last = 0;
        let statement = data.as_str().chars();
        let size = data.len();
        //println!("check");

        let mut i = 0;
        let mut token = String::new();
        let mut before_ch = '\0';
        let mut isComment = false;

        for now_ch in statement {
            if isComment && !( '\n' == now_ch || '\0' == now_ch ) {
                token.push(now_ch);
            }
            else if isComment && ( '\n' == now_ch || '\0' == now_ch ) {
                let temp = token.clone();
                result.push_back(temp); token.clear();
                isComment = false;
            }
            else if 0 == state && '\"' == now_ch  {
                token.push(now_ch);
                state = 1;
                token_last = i;
            }
            else if 1 == state && '\\' == before_ch  && '\"' == now_ch {
                token.push(now_ch);
                token_last = i;
            }
            else if 1 == state && '\"' == now_ch {
                token.push(now_ch);
                state = 0; token_last = i;

                //result.push_back(substr(statement, token_first, token_last - token_first + 1));
                //token_first = i + 1;
            }
            else if 0 == state && '=' == now_ch {
                token_last = i - 1;
                if token_last >= 0 && token_last - token_first + 1 > 0 {
                    let temp = token.clone();
                    result.push_back(temp); token.clear();
                }
                result.push_back(String::from("="));
                token_first = i + 1;
            }
            else if 0 == state && isWhitespace(now_ch) { // isspace ' ' \t \r \n , etc... ?
                token_last = i - 1;
                if token_last >= 0 && token_last - token_first + 1 > 0 {
                    let temp = token.clone();
                    result.push_back(temp); token.clear();
                }
                token_first = i + 1;
            }
            else if 0 == state && '{' == now_ch {
                token_last = i - 1;
                if token_last >= 0 && token_last - token_first + 1 > 0 {
                    let temp = token.clone();
                    result.push_back(temp); token.clear();
                }

                result.push_back(String::from("{"));
                token_first = i + 1;
            }
            else if 0 == state && '}' == now_ch {
                token_last = i - 1;
                if token_last >= 0 && token_last - token_first + 1 > 0 {
                    let temp = token.clone();
                    result.push_back(temp); token.clear();
                }
                result.push_back(String::from("}"));
                token_first = i + 1;
            }
            else if 0 == state && '#' == now_ch { // different from load_data_from_file
                token_last = i - 1;
                if token_last >= 0 && token_last - token_first + 1 > 0 {
                    let temp = token.clone();
                    result.push_back(temp); token.clear();
                }

                isComment = true;
            }
            else {
                token.push(now_ch);
            }

            i = i + 1;
            before_ch = now_ch;
        }

        if token.len() > 0
        {
            let temp = token.clone();
            result.push_back(temp); token.clear();
        }
    }
}

