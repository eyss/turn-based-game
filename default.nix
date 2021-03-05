let 
  holonixPath = builtins.fetchTarball {
    url = "https://github.com/holochain/holonix/archive/cdf1d199d5489ebc943b88e552507f1063e3e571.tar.gz";
    sha256 = "1b5pdlxj91syg1qqf42f49sxlq9qd3qnz7ccgdncjvhdfyricagh";
  };
  holonix = import (holonixPath) {
    includeHolochainBinaries = true;
    holochainVersionId = "custom";
    
    holochainVersion = { 
     rev = "5f1d6f410a185689ca670a0ea758fc80d5bcd0f5";  
     sha256 = "1kgxyfrwmga27mqywhivn0xdi6br90bavqvnd4kbrfyzbzmf8fcr";  
     cargoSha256 = "1ix8ihlizjsmx8xaaxknbl0wkyck3kc98spipx5alav8ln4wf46s";
     bins = {
       holochain = "holochain";
     };
    };
  };
in holonix.main