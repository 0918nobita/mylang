var searchIndex = JSON.parse('{\
"ast":{"doc":"","t":[0,0,0,0,13,4,13,13,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,3,11,11,11,11,12,11,11,11,11,11,11,12,11,11,11,11,11,11,8,3,11,11,11,11,11,11,12,11,11,11,11,11,10,11,11,12,11,11,11,11,11,13,13,4,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12],"n":["expr","pos","range","stmt","Add","Expr","I32Lit","StrLit","borrow","borrow_mut","deserialize","fmt","from","into","locate","serialize","try_from","try_into","type_id","0","0","0","1","1","1","Pos","borrow","borrow_mut","clone","clone_into","column","default","deserialize","fmt","fmt","from","into","line","serialize","to_owned","to_string","try_from","try_into","type_id","Locatable","Range","borrow","borrow_mut","clone","clone_into","default","deserialize","end","fmt","fmt","from","from","into","locate","new","serialize","start","to_owned","to_string","try_from","try_into","type_id","PrintI32","PrintStr","Stmt","borrow","borrow_mut","deserialize","fmt","from","into","locate","serialize","try_from","try_into","type_id","0","0","1","1"],"q":["ast","","","","ast::expr","","","","","","","","","","","","","","","ast::expr::Expr","","","","","","ast::pos","","","","","","","","","","","","","","","","","","","ast::range","","","","","","","","","","","","","","","","","","","","","","","ast::stmt","","","","","","","","","","","","","","ast::stmt::Stmt","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,2,3,4,2,3,4,0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,0,0,6,6,6,6,6,6,6,6,6,6,6,6,7,6,6,6,6,6,6,6,6,8,8,0,8,8,8,8,8,8,8,8,8,8,8,9,10,9,10],"f":[null,null,null,null,null,null,null,null,[[]],[[]],[[],["result",4]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["range",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,null,[[]],[[]],[[],["pos",3]],[[]],null,[[],["pos",3]],[[],["result",4]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],null,[[],["result",4]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,[[]],[[]],[[],["range",3]],[[]],[[],["range",3]],[[],["result",4]],null,[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[["pos",3]]],[[]],[[],["range",3]],[[["pos",3],["pos",3]]],[[],["result",4]],null,[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,[[]],[[]],[[],["result",4]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["range",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null],"p":[[4,"Expr"],[13,"I32Lit"],[13,"StrLit"],[13,"Add"],[3,"Pos"],[3,"Range"],[8,"Locatable"],[4,"Stmt"],[13,"PrintI32"],[13,"PrintStr"]]},\
"ast_interp":{"doc":"","t":[5,5],"n":["eval","execute"],"q":["ast_interp",""],"d":["",""],"i":[0,0],"f":[[[["expr",4]],["result",6,[["entity",4]]]],[[],["result",6]]],"p":[]},\
"ast_to_bytecode":{"doc":"","t":[5,5,5],"n":["ast_to_bytecode","expr_to_bytecode","stmt_to_bytecode"],"q":["ast_to_bytecode","",""],"d":["","",""],"i":[0,0,0],"f":[[[],["vec",3,[["inst",4]]]],[[["expr",4]],["vec",3,[["inst",4]]]],[[["stmt",4]],["vec",3,[["inst",4]]]]],"p":[]},\
"bytecode":{"doc":"","t":[13,13,4,13,13,13,11,11,11,11,11,11,11,11,11,11,12,12],"n":["I32Add","I32Const","Inst","PrintI32","PrintStr","StrConst","borrow","borrow_mut","deserialize","fmt","from","into","serialize","try_from","try_into","type_id","0","0"],"q":["bytecode","","","","","","","","","","","","","","","","bytecode::Inst",""],"d":["","","","","","","","","","","","","","","","","",""],"i":[1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,2,3],"f":[null,null,null,null,null,null,[[]],[[]],[[],["result",4]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null],"p":[[4,"Inst"],[13,"I32Const"],[13,"StrConst"]]},\
"bytecode_interp":{"doc":"","t":[3,11,11,11,11,11,11,12,11,11,11,5,11,11,11,11],"n":["Opts","augment_args","augment_args_for_update","borrow","borrow_mut","from","from_arg_matches","input","into","into_app","into_app_for_update","main","try_from","try_into","type_id","update_from_arg_matches"],"q":["bytecode_interp","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","",""],"i":[0,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1],"f":[null,[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[]],[[]],[[]],[[["argmatches",3]],["result",4,[["error",3]]]],null,[[]],[[],["app",3]],[[],["app",3]],[[],["result",6]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["argmatches",3]],["result",4,[["error",3]]]]],"p":[[3,"Opts"]]},\
"entity":{"doc":"","t":[12,12,4,13,3,13,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12],"n":["0","0","Entity","I32","I32Entity","Str","StrEntity","add","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","eq","eq","fmt","fmt","fmt","from","from","from","into","into","into","ne","ne","new","new","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","0","0"],"q":["entity","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","entity::Entity",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,2,0,3,0,3,0,1,1,2,3,1,2,3,1,2,1,2,3,1,2,3,1,2,3,1,2,1,2,1,2,3,1,2,3,1,2,3,4,5],"f":[null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["i32entity",3]],["bool",15]],[[["strentity",3]],["bool",15]],[[["formatter",3]],["fmtresult",6]],[[["formatter",3]],["fmtresult",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[["i32entity",3]],["bool",15]],[[["strentity",3]],["bool",15]],[[["i32",15]]],[[["string",3]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null],"p":[[3,"I32Entity"],[3,"StrEntity"],[4,"Entity"],[13,"I32"],[13,"Str"]]},\
"lexer":{"doc":"字句解析器","t":[3,8,11,11,11,11,11,12,5,10,11,0,0,12,0,5,11,11,11,0,13,13,13,4,6,13,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,13,13,13,4,13,11,11,11,11,11,11,0,11,0,0,11,11,11,11,12,12,12,3,12,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,3,12,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,3,12,11,11,11,11,12,12,11,11,11,11,12,11,11,11,11,11,11,0,0,0,0,5,13,4,13,11,11,11,5,11,11,11,11,12,12,13,13,4,11,11,11,11,5,11,11,11,12,12,13,13,13,4,11,11,11,11,5,11,11,11,12,12,12,12,3,8,11,11,12,11,11,11,11,11,12,11,11,11,10],"n":["Lex","LexExt","borrow","borrow_mut","from","into","into_iter","iter","lex","lex","next","result","state","state","transition","transition","try_from","try_into","type_id","with_pos","ForbiddenChar","InvalidEscapeSequence","InvalidKeyword","LexErr","LexResult","MissingClosingQuoteForStr","borrow","borrow_mut","fmt","fmt","from","into","locate","to_string","try_from","try_into","type_id","0","0","0","0","1","1","1","I32","Initial","Keyword","State","Str","borrow","borrow_mut","clone","clone_into","fmt","from","i32","into","keyword","str","to_owned","try_from","try_into","type_id","0","0","0","I32State","acc","append_digit_char","borrow","borrow_mut","clone","clone_into","end","fmt","from","into","new","start","to_owned","tokenize","try_from","try_into","type_id","KeywordState","acc","append_char","borrow","borrow_mut","clone","clone_into","end","fmt","from","into","new","start","to_owned","try_from","try_into","try_tokenize","type_id","StrState","acc","borrow","borrow_mut","clone","clone_into","end","escape","fmt","from","into","new","start","to_owned","tokenize","try_append_char","try_from","try_into","type_id","default","i32","keyword","str","default_transition","Continued","I32LexResult","Interrupted","borrow","borrow_mut","from","i32_lex","into","try_from","try_into","type_id","0","0","Continued","Interrupted","KeywordLexResult","borrow","borrow_mut","from","into","keyword_lex","try_from","try_into","type_id","0","0","Completed","Continued","Err","StrLexResult","borrow","borrow_mut","from","into","str_lex","try_from","try_into","type_id","0","0","0","1","WithPos","WithPosExt","borrow","borrow_mut","chars","from","into","into_iter","lex","next","pos","try_from","try_into","type_id","with_pos"],"q":["lexer","","","","","","","","","","","","","","","","","","","","lexer::result","","","","","","","","","","","","","","","","","lexer::result::LexErr","","","","","","","lexer::state","","","","","","","","","","","","","","","","","","","lexer::state::State","","","lexer::state::i32","","","","","","","","","","","","","","","","","","lexer::state::keyword","","","","","","","","","","","","","","","","","","lexer::state::str","","","","","","","","","","","","","","","","","","","lexer::transition","","","","lexer::transition::default","lexer::transition::i32","","","","","","","","","","","lexer::transition::i32::I32LexResult","","lexer::transition::keyword","","","","","","","","","","","lexer::transition::keyword::KeywordLexResult","","lexer::transition::str","","","","","","","","","","","","lexer::transition::str::StrLexResult","","","","lexer::with_pos","","","","","","","","","","","","","",""],"d":["","","","","","","","","字句解析を実行する","","","字句解析の結果","字句解析器の状態","","字句解析器内部の状態遷移の実装","","","","","…","","","","字句解析中に発生するエラー","…","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","…","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,1,1,1,1,1,0,2,1,0,0,1,0,0,1,1,1,0,3,3,3,0,0,3,3,3,3,3,3,3,3,3,3,3,3,4,5,6,7,4,5,6,8,8,8,0,8,8,8,8,8,8,8,0,8,0,0,8,8,8,8,9,10,11,0,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,0,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,0,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,0,0,0,0,0,15,0,15,15,15,15,0,15,15,15,15,16,17,18,18,0,18,18,18,18,0,18,18,18,19,20,21,21,21,0,21,21,21,21,0,21,21,21,22,23,24,24,0,0,25,25,25,25,25,25,25,25,25,25,25,25,26],"f":[null,null,[[]],[[]],[[]],[[]],[[]],null,[[]],[[],["lex",3]],[[],["option",4]],null,null,null,null,[[["state",4]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,null,[[]],[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["range",3]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[],["state",4]],[[]],[[["formatter",3]],["result",6]],[[]],null,[[]],null,null,[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,[[["pos",3],["char",15]]],[[]],[[]],[[],["i32state",3]],[[]],null,[[["formatter",3]],["result",6]],[[]],[[]],[[["pos",3],["string",3]]],null,[[]],[[],["token",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,[[["pos",3],["char",15]]],[[]],[[]],[[],["keywordstate",3]],[[]],null,[[["formatter",3]],["result",6]],[[]],[[]],[[["pos",3],["string",3]]],null,[[]],[[],["result",4]],[[],["result",4]],[[],["lexresult",6]],[[],["typeid",3]],null,null,[[]],[[]],[[],["strstate",3]],[[]],null,null,[[["formatter",3]],["result",6]],[[]],[[]],[[["pos",3]]],null,[[]],[[],["token",4]],[[["pos",3],["char",15]],["result",4,[["lexerr",4]]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,[[]],null,null,null,[[]],[[]],[[]],[[["i32state",3]],["i32lexresult",4]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,[[]],[[]],[[]],[[]],[[["keywordstate",3]],["keywordlexresult",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[["strstate",3]],["strlexresult",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,[[]],[[]],null,[[]],[[]],[[]],[[],["lex",3]],[[],["option",4]],null,[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["withpos",3]]],"p":[[3,"Lex"],[8,"LexExt"],[4,"LexErr"],[13,"ForbiddenChar"],[13,"InvalidEscapeSequence"],[13,"InvalidKeyword"],[13,"MissingClosingQuoteForStr"],[4,"State"],[13,"I32"],[13,"Str"],[13,"Keyword"],[3,"I32State"],[3,"KeywordState"],[3,"StrState"],[4,"I32LexResult"],[13,"Continued"],[13,"Interrupted"],[4,"KeywordLexResult"],[13,"Continued"],[13,"Interrupted"],[4,"StrLexResult"],[13,"Continued"],[13,"Completed"],[13,"Err"],[3,"WithPos"],[8,"WithPosExt"]]},\
"lsp":{"doc":"","t":[4,13,13,13,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12],"n":["LspMessage","Notification","Request","Response","borrow","borrow_mut","deserialize","fmt","from","into","raw_message","serialize","try_from","try_into","type_id","id","id","method","method","params","params","result"],"q":["lsp","","","","","","","","","","","","","","","lsp::LspMessage","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","",""],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,3,2,4,2,4,3],"f":[null,null,null,null,[[]],[[]],[[],["result",4]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,null],"p":[[4,"LspMessage"],[13,"Request"],[13,"Response"],[13,"Notification"]]},\
"lsp_server":{"doc":"","t":[5,0,0,0,0,0,12,3,11,11,11,11,11,11,11,3,11,11,11,11,12,11,11,11,11,3,11,11,12,11,11,11,5,5,5,5,11,12,5,11,11,11,12,3,11,11,11,11,11,11,11,3,11,11,11,11,11,11,11,11],"n":["launch_lsp_server","received_msg","receiver","responder","send_msg","sender","0","LspReceiveMsg","borrow","borrow_mut","from","into","try_from","try_into","type_id","LspReceiveActor","borrow","borrow_mut","from","into","responder","started","try_from","try_into","type_id","Responder","borrow","borrow_mut","diagnostics_supported","from","handle","into","lex","lex_and_report_errs","lex_err_to_diagnostic","locatable_to_json_range","new","sender","text_document_uri","try_from","try_into","type_id","0","LspSendMsg","borrow","borrow_mut","from","into","try_from","try_into","type_id","LspSendActor","borrow","borrow_mut","from","handle","into","try_from","try_into","type_id"],"q":["lsp_server","","","","","","lsp_server::received_msg","","","","","","","","","lsp_server::receiver","","","","","","","","","","lsp_server::responder","","","","","","","","","","","","","","","","","lsp_server::send_msg","","","","","","","","","lsp_server::sender","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,1,0,1,1,1,1,1,1,1,0,2,2,2,2,2,2,2,2,2,0,3,3,3,3,3,3,0,0,0,0,3,3,0,3,3,3,4,0,4,4,4,4,4,4,4,0,5,5,5,5,5,5,5,5],"f":[[[]],null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[]],[[]],[[]],[[]],null,[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[]],[[]],null,[[]],[[["lspreceivemsg",3]]],[[]],[[["str",15]]],[[["addr",3,[["lspsendactor",3]]],["jsonvalue",4],["str",15]]],[[["lexerr",4]],["jsonvalue",4]],[[],["jsonvalue",4]],[[["addr",3,[["lspsendactor",3]]]]],null,[[["jsonvalue",4]],["option",4,[["jsonvalue",4]]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[]],[[]],[[]],[[["lspsendmsg",3],["context",3]]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]]],"p":[[3,"LspReceiveMsg"],[3,"LspReceiveActor"],[3,"Responder"],[3,"LspSendMsg"],[3,"LspSendActor"]]},\
"parser":{"doc":"構文解析器","t":[13,13,4,13,13,11,11,5,11,11,11,11,5,5,5,5,11,11,11,11],"n":["AddOpExpected","KeywordExpected","ParseErr","TermExpected","UnexpectedToken","borrow","borrow_mut","expr","fmt","fmt","from","into","parse","program","stmt","term","to_string","try_from","try_into","type_id"],"q":["parser","","","","","","","","","","","","","","","","","","",""],"d":["","","構文解析中に発生するエラー","","","","","","","","","","","","","","","","",""],"i":[1,1,0,1,1,1,1,0,1,1,1,1,0,0,0,0,1,1,1,1],"f":[null,null,null,null,null,[[]],[[]],[[["putback",3]],["result",4,[["expr",4],["parseerr",4]]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["vec",3,[["result",4,[["stmt",4],["parseerr",4]]]]]],[[["putback",3]],["vec",3,[["result",4,[["stmt",4],["parseerr",4]]]]]],[[["putback",3]],["result",4,[["stmt",4],["parseerr",4]]]],[[],["result",4,[["expr",4],["parseerr",4]]]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]]],"p":[[4,"ParseErr"]]},\
"token":{"doc":"","t":[13,13,13,4,13,13,13,13,4,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12],"n":["AddOp","I32","Keyword","KeywordKind","Newline","PrintI32","PrintStr","Str","Token","borrow","borrow","borrow_mut","borrow_mut","deserialize","deserialize","fmt","fmt","from","from","into","into","locate","parse","serialize","serialize","try_from","try_from","try_into","try_into","type_id","type_id","0","0","0","0","0","1","1","1"],"q":["token","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","token::Token","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,1,1,0,1,2,2,1,0,2,1,2,1,2,1,2,1,2,1,2,1,1,2,2,1,2,1,2,1,2,1,3,4,5,6,7,3,5,6],"f":[null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[],["range",3]],[[["str",15]],["option",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null],"p":[[4,"Token"],[4,"KeywordKind"],[13,"I32"],[13,"AddOp"],[13,"Str"],[13,"Keyword"],[13,"Newline"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};