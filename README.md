<p align="center">
  <img src="https://user-images.githubusercontent.com/91540388/177731785-140a5403-1535-45d1-9605-eec319015bf2.png" />
</p>

Winbean is a post-exploitation tool written in Rust harvesting credentials on the machine. 
It's looking through files based on pattern, file extension, filename, specific words or function used.

Functionnalities : 
  - Enumerating network (local IPv4, DNS server, ports in used on the machine)
  - Enumerating system informations (OS, kernel version, username, hostname and language)
  - Enumerating powershell version available
  - Enumerating processes in use (PID, state, name, path)
  
  • Harvesting for credentials in file based on the context :
  - For the text file on the desktop, it is filtering for all the words based on the regex "\w*[a-z]\w*[0-9]" (DIGIT and ALPHA) and with a length minimum of 8. It is also filtering some pattern to avoid too much false positif (like URL or files paths)
  - For the others text files, it is looking for the mention of "password", "pass", "mot de passe", etc...
  - For the PHP files it is looking for some specifics functions. These functions are passing passwords as argument :
      - cubrid_connect()
      - ibase_connect()
      - fbird_pconnect()
      - db2_pconnect()
      - mysqli::real_connect()
      - mysql_connect()
      - oci_connect()
      - pg_connect()
      - sqlsrv_connect()
  It is also looking If "pass" is mentionned in the script
  - For .bash_history (If the user uses WSL for example), looking for mention of SSH
  - For Windows script (powershell and batch), looking for "-p", "-ssh", "password", "mysql". Filtering If the line starts by a "Rem" or a "#" to avoid false positive with comments. At the end it shows also all the script path files found on the machine.
  - Looking for SSH key based on the filename (id_rsa, id_dsa, etc...)
  - Looking through config file, looking for password mentions
  - Looking through specific file that might contains passwords :
    - web.config
    - Groups.xml
    - Service.xml
    - Scheduledtasks.xml
    - Datasource.xml
    - Printers.xml
    - Drives.xml 
    - error.log
    - access.log
    - applicationHost.config
    - vnc.ini
    - ultravnc.ini
    - sysprep.xml
  - Looking for go and python files
