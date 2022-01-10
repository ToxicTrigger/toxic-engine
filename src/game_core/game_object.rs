use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]
pub struct GameObject {
    pub uid: u128,  //UID Pool 에서 사용 가능한 UID 를 발급 받도록.. 죽은 객체의 UID 는 반환해서 재사용하도록 구현
    pub name: String // 식별용
    //컴포넌트 벡터가 필요함.
}