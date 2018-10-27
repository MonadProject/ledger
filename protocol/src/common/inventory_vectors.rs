use basictype::hash::Hash256;
//see https://en.bitcoin.it/wiki/Protocol_documentation#Inventory_Vectors

//Inventory vectors are used for notifying other nodes about objects they have or data which is being requested.

pub struct InventoryVectors {
    //avoid conflict
    pub types: u32,
    pub hash: Hash256,

}