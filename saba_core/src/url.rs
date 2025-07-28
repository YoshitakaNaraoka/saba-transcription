use alloc::string::String;
use alloc::vec::Vec;

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
  pub fn host(&self) -> String {
    self.host.clone()
  }
  pub fn port(&self) -> String {
    self.port.clone()
  }
  pub fn path(&self) -> String {
    self.path.clone()
  }
  pub fn searchpart(&self) -> String {
    self.searchpart.clone()
  }
    
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
      return Err("Only HTTP scheme is supported.".to_string());
    }
    self.host = self.extract_host();
    self.port = self.extract_port();
    self.path = self.extract_path();
    self.searchpart = self.extract_searchpart();
    Ok(self.clone())
  }

  fn extract_host(&self) -> String {
    /* ""内を除去した上ではじめのスラッシュまでを
      文字列スライスベクタとして取り出す */
    let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2, "/").collect();
    // ポート番号判定，場所を返すのでポート番号直前までホストとして返す
    if let Some(index) = url_parts[0].find(':') { 
      url_parts[0][..index].to_string()
    } else {
      url_parts[0].to_string()
    }
  }
  
  fn extract_port(&self) -> String {
    let url_parts:Vec<&str> = self.url.trim_start_matches("http://").splitn(2, "/").collect();
    if let Some(index) = url_parts[0].find(':') {
      url_parts[0][index+1..].to_string()
    } else {
      // ポート番号が存在しない場合，HTTPデフォルトの80を返す
      "80".to_string()
    }
  }

  fn extract_path(&self) -> String {
    let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2, "/").collect();
    if url_parts.len() < 2 { // パスが存在しない場合
      return "".to_string();
    }

    let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
    path_and_searchpart[0].to_string()
  }


  fn extract_searchpart(&self) -> String {
    let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2, "/").collect();
    if url_parts.len() < 2 {
      return "".to_string();
    }

    let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
    if path_and_searchpart.len() < 2 {
      return "".to_string();
    } else {
      path_and_searchpart[1].to_string()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // 成功ケース
  #[test]
  fn test_url_host() {
    let url = "http://example.com".to_string();
    let expected = Ok(Url {
      url: url.clone(),
      host: "example.com".to_string(),
      port: "80".to_string(),
      path: "".to_string(),
      searchpart: "".to_string(),
    });
    assert_eq!(expected, Url::new(url).parse());
    
  }

  #[test]
  fn test_url_host_port() {
    let url = "http://example.com:8888".to_string();
    let expected = Ok(Url {
      url: url.clone(),
      host: "example.com".to_string(),
      port: "8888".to_string(),
      path: "".to_string(),
      searchpart: "".to_string(),
    });
    assert_eq!(expected, Url::new(url).parse());
  }

  #[test]
  fn test_url_host_port_path() {
    let url = "http://example.com:8888/index.html".to_string();
    let expected = Ok(Url {
      url: url.clone(),
      host: "example.com".to_string(),
      port: "8888".to_string(),
      path: "index.html".to_string(),
      searchpart: "".to_string(),
    });
    assert_eq!(expected, Url::new(url).parse());
  }

  #[test]
  fn test_url_host_path() {
    let url = "http://example.com/index.html".to_string();
    let expected = Ok(Url {
      url: url.clone(),
      host: "example.com".to_string(),
      port: "80".to_string(),
      path: "index.html".to_string(),
      searchpart: "".to_string(),
    });
    assert_eq!(expected, Url::new(url).parse());
  }

  #[test]
  fn test_url_host_port_path_searchquery() {
    let url = "http://example.com:8888/index.html?a=123&b=456".to_string();
    let expected = Ok(Url {
      url: url.clone(),
      host: "example.com".to_string(),
      port: "8888".to_string(),
      path: "index.html".to_string(),
      searchpart: "a=123&b=456".to_string(),
    });
    assert_eq!(expected, Url::new(url).parse());
  }

  // 失敗ケース
  #[test]
  fn test_no_scheme() {
    let url = "example.com".to_string();
    let expected = Err("Only HTTP scheme is supported.".to_string());
    assert_eq!(expected, Url::new(url).parse());
  }

  #[test]
  fn test_unsupported_scheme() {
    let url = "https://example.com:8888/index.html".to_string();
    let expected = Err("Only HTTP scheme is supported.".to_string());
    assert_eq!(expected, Url::new(url).parse());
  }

}