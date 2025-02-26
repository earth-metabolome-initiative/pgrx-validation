# Use the official PostgreSQL image from the Docker Hub
FROM postgres:17
EXPOSE 6767
ENV DATABASE_USER="usr"
ENV DATABASE_PASSWORD="password"
ENV DATABASE_PORT=6767
ENV DATABASE_NAME="test_db"

# # Update default packages
RUN apt-get update && apt-get upgrade
RUN apt-get install -y tmux
# # Get Ubuntu packages
# RUN apt-get install -y \
#     build-essential \
#     curl

COPY my_own_extension/usr/share/postgresql/17/extension/* /usr/share/postgresql/17/extension
COPY my_own_extension/usr/lib/postgresql/17/lib/* /usr/lib/postgresql/17/lib


CMD tmux new-session -s pgrx_validation 'bash'