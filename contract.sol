pragma solidity >=0.5.0 <0.6.0;

contract ZombieFactory {
    uint myUnsignedInteger = 100;
    uint dnaDigits = 16;
    uint dnaModuls = 10 ** dnaDigits;
    
    struct Zombie {
        string name;
        uint dna;
        
        
    Zombie[] public zombies;    
    
     
    function _createZombie (string memory _name, uint _dna) private {
        zombies.push(Zombie(_name, _dna));
    }
    
    function _generateRandomDna(string memory _str) private view returns (uint) {
        uint rand = uint(keccak256(abi.encodePacked(_str)));
        return rand % dnaModulus;
    }
       

    function createRandomZombie(string memory _name) public {
        uint randDna = _generateRandomDna(_name);
        _createZombie(_name, randDna);
    }

    function createRandomZombie(string mmory _name) public {
        uint randDna = _generateRandom(_name);
        _createZombie(_name, randDna);
    }

}
