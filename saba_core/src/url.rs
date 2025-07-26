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

impl Url { 
    
  pub fn new(url: String) -> Self { // コンストラクタ内で文字列予約しておく
    Self {
      url,
      host:"".to_string(),
      port:"".to_string(),
      path:"".to_string(),
      searchpart:"".to_string(),
    }
  }

  fn is_http(&mut self) -> bool {
    if self.url.contains("http://") {
      return true;
    }
    false
  }

  pub fn parse(&mut self) -> Result<Self, String> { 
    // 自己可変参照により new 関数で作成した空文字のフィールドを変更可能
    if !self.is_http() { // まず上の関数でURLの頭を判定する
      return Err("Only HTTP scheme is supported".to_string());
    }
    {
      Ok(self.clone()) // 次のセクション以降で実装
    }
  }

}
