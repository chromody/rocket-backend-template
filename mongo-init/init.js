db = db.getSiblingDB("rocket-template");

// 1) Create the collection
db.createCollection("users");

// 2) Insert an initial document
db.users.insertOne({
  name: "Gbus",
  email: "privacy@gmail.com",
  password: "$2b$12$tgZVHMHsF1ze3pQTQhC3UegI91rQfffnpCVifVI58CBrFpKQznOZK"
});
