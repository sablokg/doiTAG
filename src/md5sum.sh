#! usr/bin/bash
# Author Gaurav Sablok,
# Email: codeprog@icloud.com
for i in *.id; do md5sum $i; done >> estimatetag.txt
rm -rf *.id
