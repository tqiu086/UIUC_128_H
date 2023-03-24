# UIUC_128_H
## Group project of UIUC 128 honors

**Group name**: Encryption project <br/>
**Member name**: Qirong Huang, Skye Qiu <br/>
**NetID**: qirongh2 tq8 <br/>

**Project goal**: we are planning to build a program encrypt and decrypt documents in docx format. The docx file is input, a text that no body would know is output, which requires a unique password to decode. We choose this goal because it can be used in our daily lives. It's interesting to send secrets to each other through communication apps with the code we write ourselves. It's also challenge enough. 

**OverView**: The program will get text from docx file, encrypt it with a random method being generated, which round the characters by ascii number in a predicable way, then output the text back to a new docx file. If time ensures, we hope it will support symmetric and insymmetric encryption. For symmetric encryption, a decryption code is put in a map with the encryption method, so that when the code is entered, the file will immediately be decrypted by doing the encryption way reversly. For insymmetric encryption, one method has one encryption code, but can't directly decrypt it. The encryption method is mapped with lots of passwords, if one password is used, it can't be used again. By using the encryption code, new passwords can be generated. 

**Checkpoints**:
- The program can read docx file and make it a string.
- The program can generate a random encryption way, and store it in a code.
- The program can generate a password and put the encryption code together in a map.
- The program can output encrypted words on docx file.
- The program can read the encrypted words, decrypt it by doing the encrypt code reversely, then return the original file. 
- The program can generate lots of one-time password for insymmetric encryption. User can easily pick symmetric/insymmetric encryption themselves.

**Possible Challenges**:
- Generate encryption code, then encrypting the words.
- Reading and outputing docx files, we can first start from txt. 
- Figure out method switching between symmetric and insymmetric encrypting method. 
