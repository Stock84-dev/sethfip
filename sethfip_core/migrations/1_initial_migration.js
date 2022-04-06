// const Migrations = artifacts.require("Migrations");
//
// module.exports = function (deployer) {
//   deployer.deploy(Migrations);
// };
//
var helloBlockchain = artifacts.require("./contracts/Storage.sol");

module.exports = function (deployer) {
	deployer.deploy(helloBlockchain)
}
