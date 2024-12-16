# Prime-Shamir
Shamir for Primes

# **Advanced Shamir's Secret Sharing with Large Primes**

This repository provides an enhanced implementation of Shamir's Secret Sharing (SSS) scheme. The design incorporates novel features such as the use of large primes, deterministic randomness, modular arithmetic with precision, and optional primality testing of shares. These features make it highly secure and suitable for advanced cryptographic applications.

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
- **Description**: Shares are optionally tested for primality, providing insights into their cryptographic relevance.
- **Use Cases**:
  - Shares may serve as cryptographic primes for advanced protocols.
  - Adds transparency to the sharing process.

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
  - Validates primality of shares (optional).

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
   - Optionally verifies the primality of the generated shares.

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

4. **Primality Testing (Optional)**:
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
Original Secret (Prime): 2233328772541849640860160335174687707160777991513379632303014434329491951347163867378456012478013664794240594519564378281957741318294401948177659821102569
Shares:
x: 1, y: 15096097757443031137886523791412500269798389113782530548817733140225636524493002368770314819052776042106334354484921175611106298613556624894249132426574955374627127719067536349560046168964716199990103645104121812453210675165564572820430553860254968867418684453451322607878147464670634714571963464887001438566
x: 2, y: 32702446687729099024460821590486652320535796103032178906948745464236850065862235079227337418119123955900960196005554729202866049332097597195747760731876868277028616999805185390729439023408100208766624071628606750912989903409080598965269076369978146150474874172796928837452011449217877816941148551097791231725
x: 3, y: 6825640841908880657979329604107815520138185200786736975959454773857742686233200466637288355668082166430176840153293443833659795334279435461194106117647055428091841025772257879412708886179171576119803977966697154281247466883728021798412890039514279927413295344349026973322238737332581654949127447372170495487
x: 4, y: 11021015289514370210400713967929922074093386422641986956375206759394919028992387321549296877219616149483502609848478472348917089638391148376579461537319485685733406877492582448595478799531569186076828083692751966700396560840527886308890595484856877594386205451803145749208891035923674308676875788945333975500
x: 5, y: 38051270993495267894019407612845942779796014335923478017557585282204414782337371444573491885628047407745666621199640502688698800301529170796772075642076159483759999095047903645509981504362269291403708847189317404552080778231832425286508415552992190428421217230658062173028156053946360859925188468257732047371
x: 6, y: 58190000204552100872837872616069323234282326497824962725594292809410790575988097471391716690879591500255241485606486676552076838989514694152998886895043155533378373297443799930556855106477574888288330389766824309565451548838867551809591799286845158519615756453164409900886537715609070610015872789408724150614
x: 7, y: 12863986725620161162754181181062677428617596976305168531685888310588184519607865380210087815011368808536615782801951069722314533704125398268352795143451510150710869152320061470747785277430360465783704989815220754644449382395274413363736366316321746107608662695492487582751106100550388236912260242259329088549
x: 8, y: 2279181565900992565303640730924709182325944622709564079041952768555488816095202951886021631665997092711947221077054487409645069038383978949223320816903598706294365294627688549585607851643550630970591866859798891538073699722653761067530829971062105705220361158242467293091568203106033134381762417587684730348
Share at x = 1 is NOT prime.
Share at x = 2 is NOT prime.
Share at x = 3 is NOT prime.
Share at x = 4 is NOT prime.
Share at x = 5 is NOT prime.
Share at x = 6 is NOT prime.
Share at x = 7 is NOT prime.
Share at x = 8 is NOT prime.
Reconstructed Secret: 2233328772541849640860160335174687707160777991513379632303014434329491951347163867378456012478013664794240594519564378281957741318294401948177659821102569
Reconstruction successful. The secret matches exactly.
```
