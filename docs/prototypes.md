# AuriFS  

### La gestion de la sécurité :
- Séparation des privilèges via les capabilities-based-security
- Vérfication systémique des droits ( fait dans le VFS )
- Pas de corruption par l'user  ( parser les pointeurs de bloc pour vérifier qu'il ne dépassent pas la tailles du volumes )
- Zéroisation ( quand un bloc contenant des données et libérer on l'écrase avec des zeros AVANT de le marquer libre dans le bitmap )

### La gestion de la discrétion :
- Minimalisation des métadonnées ( timestamps d'accès uniquement activé si le montage l'exige par exemple)
- Noms chiffrés 
- Padding
- Journaling chiffrés
