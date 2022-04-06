pragma solidity >=0.4.22 <0.9.0;

contract Storage {
    string cid;

    function set(string memory x) public {
        cid = x;
    }

    function get() public returns (string memory) {
        return cid;
    }
}
