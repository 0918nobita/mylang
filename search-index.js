var searchIndex = JSON.parse('{\
"ast":{"doc":"抽象構文木の定義","t":[13,4,13,13,13,4,13,0,0,12,12,12,12,12,12,12,12,12,12,13,4,13,13,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,13,13,4,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12],"n":["Add","Expr","I32Lit","PrintI32","PrintStr","Stmt","StrLit","expr","stmt","0","0","0","1","1","1","0","0","1","1","Add","Expr","I32Lit","StrLit","borrow","borrow_mut","deserialize","fmt","from","into","locate","serialize","try_from","try_into","type_id","0","0","0","1","1","1","PrintI32","PrintStr","Stmt","borrow","borrow_mut","deserialize","fmt","from","into","locate","serialize","try_from","try_into","type_id","0","0","1","1"],"q":["ast","","","","","","","","","ast::Expr","","","","","","ast::Stmt","","","","ast::expr","","","","","","","","","","","","","","","ast::expr::Expr","","","","","","ast::stmt","","","","","","","","","","","","","","ast::stmt::Stmt","","",""],"d":["","式を表す抽象構文木","","","","文を表す抽象構文木","","","","","","","","","","","","","","","式を表す抽象構文木","","","","","","","","","","","","","","","","","","","","","","文を表す抽象構文木","","","","","","","","","","","","","","",""],"i":[1,0,1,2,2,0,1,0,0,3,4,5,3,4,5,6,7,6,7,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,3,4,5,3,4,5,2,2,0,2,2,2,2,2,2,2,2,2,2,2,6,7,6,7],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[],["result",4]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["range",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,[[]],[[]],[[],["result",4]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["range",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null],"p":[[4,"Expr"],[4,"Stmt"],[13,"I32Lit"],[13,"StrLit"],[13,"Add"],[13,"PrintI32"],[13,"PrintStr"]]},\
"ast_interp":{"doc":"抽象構文木インタプリタ","t":[5,5],"n":["eval","execute"],"q":["ast_interp",""],"d":["",""],"i":[0,0],"f":[[[["expr",4]],["result",6,[["entity",4]]]],[[],["result",6]]],"p":[]},\
"ast_to_bytecode":{"doc":"抽象構文木→バイトコード変換器","t":[5,5,5],"n":["ast_to_bytecode","expr_to_bytecode","stmt_to_bytecode"],"q":["ast_to_bytecode","",""],"d":["","",""],"i":[0,0,0],"f":[[[],["vec",3,[["inst",4]]]],[[["expr",4]],["vec",3,[["inst",4]]]],[[["stmt",4]],["vec",3,[["inst",4]]]]],"p":[]},\
"bytecode":{"doc":"バイトコードの定義","t":[13,13,4,13,13,13,11,11,11,11,11,11,11,11,11,11,12,12],"n":["I32Add","I32Const","Inst","PrintI32","PrintStr","StrConst","borrow","borrow_mut","deserialize","fmt","from","into","serialize","try_from","try_into","type_id","0","0"],"q":["bytecode","","","","","","","","","","","","","","","","bytecode::Inst",""],"d":["","","","","","","","","","","","","","","","","",""],"i":[1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,2,3],"f":[null,null,null,null,null,null,[[]],[[]],[[],["result",4]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null],"p":[[4,"Inst"],[13,"I32Const"],[13,"StrConst"]]},\
"bytecode_interp":{"doc":"バイトコードインタプリタ","t":[4,6,13,13,11,11,5,11,11,11,5,5,11,5,5,5,11,11,11,11,12],"n":["InterpError","InterpResult","StackUnderflow","TypeMismatch","borrow","borrow_mut","execute","fmt","fmt","from","i32_add","i32_const","into","print_i32","print_str","str_const","to_string","try_from","try_into","type_id","0"],"q":["bytecode_interp","","","","","","","","","","","","","","","","","","","","bytecode_interp::InterpError"],"d":["…","バイトコードの実行結果","…","…","","","バイトコードを解釈実行する","","","","I32Add 命令を実行する","I32Const 命令を実行する","","PrintI32 命令を実行する","PrintStr 命令を実行する","StrConst 命令を実行する","","","","",""],"i":[0,0,1,1,1,1,0,1,1,1,0,0,1,0,0,0,1,1,1,1,2],"f":[null,null,null,null,[[]],[[]],[[],["result",4,[["interperror",4]]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[["vec",3]],["result",4,[["interperror",4]]]],[[["vec",3],["i32",15]]],[[]],[[["vec",3]],["result",4,[["interperror",4]]]],[[["vec",3]],["result",4,[["interperror",4]]]],[[["vec",3],["str",15]]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null],"p":[[4,"InterpError"],[13,"StackUnderflow"]]},\
"entity":{"doc":"mylang のランタイムで扱う「実体」の定義","t":[12,12,4,13,3,13,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12],"n":["0","0","Entity","I32","I32Entity","Str","StrEntity","add","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","eq","eq","fmt","fmt","fmt","fmt","fmt","from","from","from","into","into","into","ne","ne","new","new","serialize","serialize","serialize","to_string","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","0","0"],"q":["entity","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","entity::Entity",""],"d":["","","バイトコード実行時に扱う値","","符号付き32ビット整数値","","文字列データ","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,2,0,3,0,3,0,1,1,2,3,1,2,3,1,2,1,1,2,2,3,1,2,3,1,2,3,1,2,1,2,1,2,3,1,2,1,2,3,1,2,3,1,2,3,4,5],"f":[null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["i32entity",3]],["bool",15]],[[["strentity",3]],["bool",15]],[[["formatter",3]],["fmtresult",6]],[[["formatter",3]],["fmtresult",6]],[[["formatter",3]],["fmtresult",6]],[[["formatter",3]],["fmtresult",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[["i32entity",3]],["bool",15]],[[["strentity",3]],["bool",15]],[[["i32",15]]],[[["string",3]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["string",3]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null],"p":[[3,"I32Entity"],[3,"StrEntity"],[4,"Entity"],[13,"I32"],[13,"Str"]]},\
"lexer":{"doc":"字句解析器","t":[13,13,13,3,4,8,13,8,11,11,11,11,11,12,5,10,11,0,0,12,0,5,11,11,11,0,10,12,12,12,12,12,12,12,13,13,13,4,6,13,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,13,13,13,4,13,11,11,11,11,11,11,0,11,0,0,11,11,11,11,12,12,12,3,12,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,3,12,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,3,12,11,11,11,11,12,12,11,11,11,11,12,11,11,11,11,11,11,0,0,0,0,5,13,4,13,11,11,11,5,11,11,11,11,12,12,13,13,4,11,11,11,11,5,11,11,11,12,12,13,13,13,4,11,11,11,11,5,11,11,11,12,12,12,12,3,8,11,11,12,11,11,11,11,11,11,12,11,11,11,10],"n":["ForbiddenChar","InvalidEscapeSequence","InvalidKeyword","Lex","LexErr","LexExt","MissingClosingQuoteForStr","WithPosExt","borrow","borrow_mut","from","into","into_iter","iter","lex","lex","next","result","state","state","transition","transition","try_from","try_into","type_id","with_pos","with_pos","0","0","0","0","1","1","1","ForbiddenChar","InvalidEscapeSequence","InvalidKeyword","LexErr","LexResult","MissingClosingQuoteForStr","borrow","borrow_mut","fmt","fmt","from","into","locate","to_string","try_from","try_into","type_id","0","0","0","0","1","1","1","I32","Initial","Keyword","State","Str","borrow","borrow_mut","clone","clone_into","fmt","from","i32","into","keyword","str","to_owned","try_from","try_into","type_id","0","0","0","I32State","acc","append_digit_char","borrow","borrow_mut","clone","clone_into","end","fmt","from","into","new","start","to_owned","tokenize","try_from","try_into","type_id","KeywordState","acc","append_char","borrow","borrow_mut","clone","clone_into","end","fmt","from","into","new","start","to_owned","try_from","try_into","try_tokenize","type_id","StrState","acc","borrow","borrow_mut","clone","clone_into","end","escape","fmt","from","into","new","start","to_owned","tokenize","try_append_char","try_from","try_into","type_id","default","i32","keyword","str","default_transition","Continued","I32LexResult","Interrupted","borrow","borrow_mut","from","i32_lex","into","try_from","try_into","type_id","0","0","Continued","Interrupted","KeywordLexResult","borrow","borrow_mut","from","into","keyword_lex","try_from","try_into","type_id","0","0","Completed","Continued","Err","StrLexResult","borrow","borrow_mut","from","into","str_lex","try_from","try_into","type_id","0","0","0","1","WithPos","WithPosExt","borrow","borrow_mut","chars","from","into","into_iter","lex","multiunzip","next","pos","try_from","try_into","type_id","with_pos"],"q":["lexer","","","","","","","","","","","","","","","","","","","","","","","","","","","lexer::LexErr","","","","","","","lexer::result","","","","","","","","","","","","","","","","","lexer::result::LexErr","","","","","","","lexer::state","","","","","","","","","","","","","","","","","","","lexer::state::State","","","lexer::state::i32","","","","","","","","","","","","","","","","","","lexer::state::keyword","","","","","","","","","","","","","","","","","","lexer::state::str","","","","","","","","","","","","","","","","","","","lexer::transition","","","","lexer::transition::default","lexer::transition::i32","","","","","","","","","","","lexer::transition::i32::I32LexResult","","lexer::transition::keyword","","","","","","","","","","","lexer::transition::keyword::KeywordLexResult","","lexer::transition::str","","","","","","","","","","","","lexer::transition::str::StrLexResult","","","","lexer::with_pos","","","","","","","","","","","","","","",""],"d":["","","","","字句解析中に発生するエラー","","","","","","","","","","字句解析を実行する","","","字句解析の結果","字句解析器の状態","","字句解析器内部の状態遷移の実装","","","","","…","","","","","","","","","","","","字句解析中に発生するエラー","…","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","…","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,1,1,0,0,0,1,0,2,2,2,2,2,2,0,3,2,0,0,2,0,0,2,2,2,0,4,5,6,7,8,5,6,7,1,1,1,0,0,1,1,1,1,1,1,1,1,1,1,1,1,5,6,7,8,5,6,7,9,9,9,0,9,9,9,9,9,9,9,0,9,0,0,9,9,9,9,10,11,12,0,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,0,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,0,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,0,0,0,0,0,16,0,16,16,16,16,0,16,16,16,16,17,18,19,19,0,19,19,19,19,0,19,19,19,20,21,22,22,22,0,22,22,22,22,0,22,22,22,23,24,25,25,0,0,26,26,26,26,26,26,26,26,26,26,26,26,26,4],"f":[null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],null,[[]],[[],["lex",3]],[[],["option",4]],null,null,null,null,[[["state",4]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[],["withpos",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["range",3]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[],["state",4]],[[]],[[["formatter",3]],["result",6]],[[]],null,[[]],null,null,[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,[[["pos",3],["char",15]]],[[]],[[]],[[],["i32state",3]],[[]],null,[[["formatter",3]],["result",6]],[[]],[[]],[[["pos",3],["string",3]]],null,[[]],[[],["token",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,[[["pos",3],["char",15]]],[[]],[[]],[[],["keywordstate",3]],[[]],null,[[["formatter",3]],["result",6]],[[]],[[]],[[["pos",3],["string",3]]],null,[[]],[[],["result",4]],[[],["result",4]],[[],["result",4,[["token",4],["lexerr",4]]]],[[],["typeid",3]],null,null,[[]],[[]],[[],["strstate",3]],[[]],null,null,[[["formatter",3]],["result",6]],[[]],[[]],[[["pos",3]]],null,[[]],[[],["token",4]],[[["pos",3],["char",15]],["result",4,[["lexerr",4]]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,[[]],null,null,null,[[]],[[]],[[]],[[["i32state",3]],["i32lexresult",4]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,[[]],[[]],[[]],[[]],[[["keywordstate",3]],["keywordlexresult",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[["strstate",3]],["strlexresult",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,[[]],[[]],null,[[]],[[]],[[]],[[],["lex",3]],[[]],[[],["option",4]],null,[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["withpos",3]]],"p":[[4,"LexErr"],[3,"Lex"],[8,"LexExt"],[8,"WithPosExt"],[13,"ForbiddenChar"],[13,"InvalidEscapeSequence"],[13,"InvalidKeyword"],[13,"MissingClosingQuoteForStr"],[4,"State"],[13,"I32"],[13,"Str"],[13,"Keyword"],[3,"I32State"],[3,"KeywordState"],[3,"StrState"],[4,"I32LexResult"],[13,"Continued"],[13,"Interrupted"],[4,"KeywordLexResult"],[13,"Continued"],[13,"Interrupted"],[4,"StrLexResult"],[13,"Continued"],[13,"Completed"],[13,"Err"],[3,"WithPos"]]},\
"lsp_server":{"doc":"mylang を記述するための、LSP …","t":[5,0,0,0,0,4,13,13,13,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,3,11,11,11,11,11,12,11,11,11,11,3,0,11,11,0,12,11,11,0,11,11,0,12,11,11,11,5,5,5,5,5,5,5,5,5,5,5,5,3,11,11,11,11,11,11,11,11],"n":["launch_lsp_server","message","receiver","responder","sender","LspMessage","Notification","Request","Response","borrow","borrow_mut","deserialize","fmt","from","into","raw_message","serialize","try_from","try_into","type_id","id","id","method","method","params","params","result","Receiver","borrow","borrow_mut","from","into","new","responder","started","try_from","try_into","type_id","Responder","analyzer","borrow","borrow_mut","diagnostic","diagnostics_supported","from","handle","handler","into","new","range","sender","try_from","try_into","type_id","analyze_src","lex","lex_err_to_diagnostic","parse_err_to_diagnostic","handle","handle_did_change_notification","handle_did_close_notification","handle_did_open_notification","handle_initialize_request","text_document_uri","token_types","locatable_to_lsp_range","Sender","borrow","borrow_mut","from","handle","into","try_from","try_into","type_id"],"q":["lsp_server","","","","","lsp_server::message","","","","","","","","","","","","","","","lsp_server::message::LspMessage","","","","","","","lsp_server::receiver","","","","","","","","","","","lsp_server::responder","","","","","","","","","","","","","","","","lsp_server::responder::analyzer","","lsp_server::responder::diagnostic","","lsp_server::responder::handler","","","","","","","lsp_server::responder::range","lsp_server::sender","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,3,2,4,2,4,3,0,5,5,5,5,5,5,5,5,5,5,0,0,6,6,0,6,6,6,0,6,6,0,6,6,6,6,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,7,7,7,7,7],"f":[[[]],null,null,null,null,null,null,null,null,[[]],[[]],[[],["result",4]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[["addr",3,[["responder",3]]]]],null,[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,[[]],[[]],null,null,[[]],[[["lspmessage",4]]],null,[[]],[[["addr",3,[["sender",3]]]]],null,null,[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["addr",3,[["sender",3]]],["jsonvalue",4],["str",15]]],[[["str",15]]],[[["lexerr",4]],["jsonvalue",4]],[[["parseerr",4]],["jsonvalue",4]],[[["addr",3,[["sender",3]]],["arc",3,[["atomicbool",3]]],["lspmessage",4]]],[[["addr",3,[["sender",3]]],["jsonvalue",4]]],[[["addr",3,[["sender",3]]],["jsonvalue",4]]],[[["addr",3,[["sender",3]]],["jsonvalue",4]]],[[["addr",3,[["sender",3]]],["arc",3,[["atomicbool",3]]],["usize",15],["jsonvalue",4]]],[[["jsonvalue",4]],["option",4,[["jsonvalue",4]]]],[[["jsonvalue",4]],["option",4,[["hashmap",3,[["string",3],["usize",15]]]]]],[[],["jsonvalue",4]],null,[[]],[[]],[[]],[[["lspmessage",4],["context",3]]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]]],"p":[[4,"LspMessage"],[13,"Request"],[13,"Response"],[13,"Notification"],[3,"Receiver"],[3,"Responder"],[3,"Sender"]]},\
"parser":{"doc":"構文解析器","t":[13,4,13,11,11,5,11,11,11,11,11,5,5,5,5,11,11,11,11,12,12],"n":["KeywordExpected","ParseErr","TermExpected","borrow","borrow_mut","expr","fmt","fmt","from","into","locate","parse","program","stmt","term","to_string","try_from","try_into","type_id","0","0"],"q":["parser","","","","","","","","","","","","","","","","","","","parser::ParseErr",""],"d":["","構文解析中に発生するエラー","","","","","","","","","","","","","","","","","","",""],"i":[1,0,1,1,1,0,1,1,1,1,1,0,0,0,0,1,1,1,1,2,3],"f":[null,null,null,[[]],[[]],[[["putback",3],["pos",3]],["result",4,[["parseerr",4]]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["range",3]],[[],["vec",3,[["result",4,[["stmt",4],["parseerr",4]]]]]],[[["putback",3]],["vec",3,[["result",4,[["stmt",4],["parseerr",4]]]]]],[[["putback",3],["pos",3]],["result",4,[["stmt",4],["parseerr",4]]]],[[["putback",3],["pos",3]],["result",4,[["parseerr",4]]]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null],"p":[[4,"ParseErr"],[13,"TermExpected"],[13,"KeywordExpected"]]},\
"token":{"doc":"…","t":[13,13,13,4,8,13,3,13,13,3,13,4,11,11,11,11,12,11,11,12,11,11,11,11,11,11,12,0,10,11,11,0,14,0,14,11,11,12,11,11,11,11,11,11,12,12,12,12,12,12,12,12,8,10,3,11,11,12,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,3,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11],"n":["AddOp","I32","Keyword","KeywordKind","Locatable","Newline","Pos","PrintI32","PrintStr","Range","Str","Token","borrow","borrow","borrow_mut","borrow_mut","character","deserialize","deserialize","end","fmt","fmt","from","from","into","into","line","locatable","locate","locate","parse","pos","pos","range","range","serialize","serialize","start","try_from","try_from","try_into","try_into","type_id","type_id","0","0","0","0","0","1","1","1","Locatable","locate","Pos","borrow","borrow_mut","character","clone","clone_into","default","deserialize","eq","fmt","fmt","from","into","line","ne","new","next_char","next_line","serialize","to_owned","to_string","try_from","try_into","type_id","Range","borrow","borrow_mut","clone","clone_into","concat","default","deserialize","end","end","end_ref","eq","fmt","fmt","from","from","into","ne","new","serialize","start","to_owned","to_string","try_from","try_into","type_id"],"q":["token","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","token::Token","","","","","","","","token::locatable","","token::pos","","","","","","","","","","","","","","","","","","","","","","","","token::range","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","…","","ソースコード上の位置","","","ソースコード中の範囲","","","","","","","列番号","","","終端","","","","","","","行番号","","対応する範囲を返す","","","","","","","","","始端","","","","","","","","","","","","","","","…","対応する範囲を返す","ソースコード上の位置","","","列番号","","","","","","","","","","行番号","","新しい位置を生成して返す","次の文字位置に移動する …","次の行の先頭に移動する","","","","","","","ソースコード中の範囲","","","","","終端を別の範囲の終端に設定する","","","所有権をムーブして終端を返す","終端","終端の参照を返す","","","","始端と終端が同じ位置の範囲に変換する","","","","新しい範囲を生成して返す","","始端","","","","",""],"i":[1,1,1,0,0,1,0,2,2,0,1,0,2,1,2,1,3,2,1,4,2,1,2,1,2,1,3,0,5,1,2,0,0,0,0,2,1,4,2,1,2,1,2,1,6,7,8,9,10,6,8,9,0,5,0,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,0,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4],"f":[null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],null,[[],["result",4]],[[],["result",4]],null,[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],null,null,[[],["range",3]],[[],["range",3]],[[["str",15]],["option",4]],null,null,null,null,[[],["result",4]],[[],["result",4]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,[[],["range",3]],null,[[]],[[]],null,[[],["pos",3]],[[]],[[],["pos",3]],[[],["result",4]],[[["pos",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],null,[[["pos",3]],["bool",15]],[[["u32",15],["u32",15]]],[[]],[[]],[[],["result",4]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[]],[[]],[[],["range",3]],[[]],[[["range",3]]],[[],["range",3]],[[],["result",4]],[[],["pos",3]],null,[[],["pos",3]],[[["range",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["pos",3]]],[[]],[[]],[[["range",3]],["bool",15]],[[["pos",3],["pos",3]]],[[],["result",4]],null,[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]]],"p":[[4,"Token"],[4,"KeywordKind"],[3,"Pos"],[3,"Range"],[8,"Locatable"],[13,"I32"],[13,"AddOp"],[13,"Str"],[13,"Keyword"],[13,"Newline"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};