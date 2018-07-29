FROM scratch
COPY md-toc /usr/bin/md-toc

ENTRYPOINT [ "md-toc" ]
