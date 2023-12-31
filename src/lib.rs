use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize,Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey};
use schemars::{schema_for, JsonSchema};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub struct StudentsManagement {
    school: String,
    student: LookupMap<AccountId, StudentMetadata>,
}

#[derive(BorshDeserialize, BorshSerialize,JsonSchema ,Serialize ,Deserialize)]
#[serde(crate="near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct StudentMetadata {
    name: String,   
    address: String,
    email: String,
    phone:String,
}

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
enum StorageKey {
    StudentsMessageKeys,
}

impl Default for StudentsManagement {
    fn default() -> Self {
        Self {
            school: "University of technology".to_string(),
            student: LookupMap::new(StorageKey::StudentsMessageKeys),
        }
    }
}

pub trait StudentFeature {
    fn Create_Student(&mut self , student_account:AccountId , name: String , address:String , email:String, phone:String);
    fn Get_Student(&self , student_account:AccountId)-> Option<StudentMetadata>;// -> call to fun student , retrurn form StudentMetadata
    fn Update_Student(&mut self , student_account:AccountId , name: Option<String> , address: Option<String> , email:Option<String>, phone: Option<String>); // option use option if use don't change it don't anything because date is store 
    fn Delete_Student(&mut self, student_account:AccountId );

}
#[near_bindgen]
impl StudentFeature for StudentsManagement {
    fn Create_Student(&mut self , student_account:AccountId , name: String , address:String , email:String, phone:String )
    {
        let new_student : StudentMetadata = StudentMetadata {name ,address,email,phone};
        self.student.insert(&student_account, &new_student);
    }

    fn Get_Student(&self , student_account:AccountId)-> Option<StudentMetadata>{
        self.student.get(&student_account)
    }

    fn Update_Student (&mut self , student_account:AccountId ,name: Option<String> , address: Option<String> , email:Option<String>, phone: Option<String>){
        let mut UP_student = self.student.get(&student_account).unwrap();
        if let Some(new_name) = name{
            UP_student.name = new_name;
        }
        if let Some(new_address) = address{
            UP_student.address = new_address;
        }
        if let Some(new_email) = email{
            UP_student.email = new_email;
        }
        if let Some(new_phone) = phone{
            UP_student.phone = new_phone;
        }

        self.student.insert(&student_account , &UP_student);
    }

    fn Delete_Student(&mut self , student_account:AccountId ){
        self.student.remove(&student_account);
    }
}
