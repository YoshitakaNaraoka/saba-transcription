use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
  url: String, // フィールドは名前と型を持つ
  host: String,
  port: String,
  path: String,
  searchpart: String,
}

use alloc::string::ToString;

impl Url { // コンストラクタ内で文字列予約しておく
  pub fn parse(&mut self) -> Result<Self, String> { 
    // 自己可変参照により new 関数で作成した空文字のフィールドを変更可能
    {
      Ok(self.clone()) // 次のセクション以降で実装
    }
  }
  
  pub fn new(url: String) -> Self {
    Self {
      url,
      host:"".to_string(),
      port:"".to_string(),
      path:"".to_string(),
      searchpart:"".to_string(),
    }
  }
}
