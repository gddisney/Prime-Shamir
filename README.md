# Prime-Shamir
**Shamir for Primes**

# **Advanced Shamir's Secret Sharing with Large Primes**

This repository provides an enhanced implementation of Shamir's Secret Sharing (SSS) scheme. The design incorporates novel features such as the use of large primes, deterministic randomness, modular arithmetic with precision, and mandatory primality testing of shares. These features make it highly secure and suitable for advanced cryptographic applications.

---

## **Features**

### **1. Large Primes for Secrets and Modulus**
- **Description**: Uses *large primes* (e.g., 512-bit and larger) as the secret and ensures the modulus is at least twice the bit size of the secret.
- **Advantages**:
  - Increased resistance to brute-force and factorization attacks.
  - Enhanced compatibility with cryptographic systems like RSA.

---

### **2. Precision in Modular Arithmetic**
- **Description**: Implements modular arithmetic rigorously to avoid wrap-around and ensure secure operations on very large secrets.
- **Key Benefits**:
  - Accurate reconstruction of the secret.
  - Secure handling of coefficients and shares during operations.

---

### **3. Primality Testing of Shares**
- **Description**: Shares are tested for primality, ensuring their cryptographic relevance.
- **Use Cases**:
  - Shares may serve as cryptographic primes for advanced protocols.
  - Adds transparency to the sharing process and improves integrity.

---

### **4. Deterministic Randomness**
- **Description**: Employs `ChaCha20Rng`, a cryptographically secure random number generator (CSPRNG), to ensure deterministic randomness.
- **Advantages**:
  - Enhanced security and reproducibility.
  - Resistant to predictable randomness attacks.

---

### **5. Efficient Reconstruction with Modular Arithmetic**
- **Description**: Uses Fermat's Little Theorem to compute modular inverses during Lagrange interpolation.
- **Key Features**:
  - Faster computations for large secrets.
  - Reduced risk of numerical instability with very large numbers.

---

### **6. Flexible Parameters**
- **Configurable Options**:
  - **Threshold (`k`)**: Minimum number of shares required for reconstruction.
  - **Total Shares (`n`)**: Total number of shares to distribute.
  - **Modulus**: Large enough to securely accommodate both the secret and the shares.

---

### **7. Error Checking and Validation**
- **Description**: Built-in assertions and validations ensure the scheme operates correctly.
- **Validation Features**:
  - Ensures the threshold is valid (`k > 1`).
  - Verifies that the number of shares is sufficient for reconstruction (`n >= k`).
  - Validates primality of shares.

---

## **Implementation Details**

### **Modules**
1. **`generate_large_prime(bits: usize) -> BigUint`**
   - Generates a large prime of the specified bit size using the Rabin-Miller primality test.

2. **`shamir_split_shares(secret: &BigUint, threshold: usize, shares: usize, modulus: &BigUint) -> Vec<(usize, BigUint)>`**
   - Splits the secret into `n` shares using a polynomial with random coefficients.

3. **`shamir_reconstruct(shares: &[(usize, BigUint)], modulus: &BigUint) -> BigUint`**
   - Reconstructs the secret using Lagrange interpolation with modular arithmetic.

4. **`verify_share_primality(shares: &[(usize, BigUint)])`**
   - Verifies the primality of the generated shares.

---

## **How It Works**

1. **Secret Generation**:
   - A large prime is generated as the secret.
   - A larger modulus is used to prevent wrap-around during computations.

2. **Share Generation**:
   - Coefficients are generated using `ChaCha20Rng` for secure randomness.
   - Each share is computed as a polynomial evaluation modulo the larger modulus.

3. **Reconstruction**:
   - The secret is reconstructed using Lagrange interpolation with modular arithmetic.

4. **Primality Testing**:
   - Each share is tested for primality to assess its potential use in advanced cryptographic protocols.

---

## **Applications**

- **Distributed Key Management**:
  Securely distribute cryptographic keys across multiple parties.

- **Multi-Party Computation**:
  Enable secure computations where no single party has access to the entire secret.

- **Layered Cryptographic Protocols**:
  Utilize prime shares in advanced cryptographic constructions like zero-knowledge proofs or homomorphic encryption.

---

## **Usage**

### **Generate a Secret and Shares**
```rust
let secret_bits = 512;
let secret = generate_large_prime(secret_bits);

let modulus_bits = secret_bits * 2;
let modulus = generate_large_prime(modulus_bits);

let threshold = 6; // Minimum shares required for reconstruction
let shares_count = 8; // Total shares to generate

let shares = shamir_split_shares(&secret, threshold, shares_count, &modulus);

println!("Original Secret: {}", secret);
println!("Shares: {:?}", shares);
```

---

### **Example Output**
```
Original Secret (Prime): 10900763121876576560626526914132743145197982112077950381571453059766177028951380140617251956527870194261492118598025600353872094784577122605526189759043459
Shares:
x: 1, y: 14158940939604300170588190466244988424686545306116268965100817199182956548684553294757657129666547214088467966992254931584590254181831314668233254133599510135517735008734512145706860801922403352534646468476165715907543518098531459660916711212375706434065546710731257060506497137782453531489990142862895476851
x: 2, y: 34831014324263026110130506035918197527470926383973367221867669128509060328745103321623627083123010752283154524345434157544552603572049775183861368682526528209910465594458056707150238999765431597796404555954500742566856112134223713418508910814943529814811939083511920031163131473924916951663929030245734864509
x: 3, y: 46476974100311296644755658363922902154658544143428135512909352310085380185781416663448260489998906337717454411586727233780718377908004235706654418242618028669318752089080679289574375744836934202527346380031710726409419678455353539233076196747820400705761398130144344890939623580585804189848254507718918921659
x: 4, y: 32648211691034629521322009379349072184949348752571304044919478604143981264789836448048964866455586085485178967493307769919494338850589653563120717508310965912019141226094246053931963215826242180664343470614402789107421456699631391189995698376623028328761161662977929796203589180843670109386287823763271177127
x: 5, y: 6486539651494628319762296091692732950863387858625033446689787646663538467747335247678364220793629149913542390481098193464936297119793127473916000862803765603415321215608393999279701084443743808163228191136686820878120411630582983122591986380605168076027209574134388191145921677523896731239575465630618231169
x: 6, y: 41223975323354669365428355397075325660166354808840455727123459286990949911195567978492458659060232371836147998993534513950822729630239935781252601968217805090266154942809920618646938848317315139840843081827395202335153848796901920854866365388838361151993727622614423892605820809930594880466625469014611011763
x: 7, y: 2114407385645809822987052209181218545576464906309876404413425522671155494106011618173377745349630950373154523779031317786019350168903421814837346338117727205733836461487449603856174884894829237896145211901160148409538375627747741245683768365256062742765557993459444873221116546309658681378870097434600142301
x: 8, y: 24785555165682142836438199278399841193768011578492323707308183691256442690042387878165649475370944053443492320446696186366616296217405261411487661454134670597381739249892906427963913702831468621233673009439846997096434998938180434229315646621539298866563182926775680298765354886983937931003562597336876020513
Share at x = 1 is prime.
Share at x = 2 is prime.
Share at x = 3 is prime.
Share at x = 4 is prime.
Share at x = 5 is prime.
Share at x = 6 is prime.
Share at x = 7 is prime.
Share at x = 8 is prime.
Reconstructed Secret: 10900763121876576560626526914132743145197982112077950381571453059766177028951380140617251956527870194261492118598025600353872094784577122605526189759043459
Reconstruction successful. The secret matches exactly.
```

