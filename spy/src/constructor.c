#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

int sock_fd = -1;

char *get_path_socket(char **envp) {
    const char env_name[] = "SCRY_SOCK_PATH=";
    int size_name = 15;

    for (char **p = envp; *p; ++p) {

        if (strncmp(*p, env_name, size_name) == 0) {
            return *p + size_name;
        }
    }
    return NULL;
}

__attribute__((constructor)) void init(int argc, char **argv, char **envp) {

    (void)argc;
    (void)argv;

    int ret;
    char *sock_path;

    struct sockaddr_un servaddr;

    sock_path = get_path_socket(envp);
    if (!sock_path) {
        write(2, "SCRY_SOCK_PATH env not found\n", 29);
        return;
    }

    // Configuration of servaddr struct
    memset(&servaddr, 0, sizeof(servaddr));
    servaddr.sun_family = AF_UNIX;
    strcpy(servaddr.sun_path, sock_path);

    sock_fd = socket(AF_UNIX, SOCK_DGRAM, 0);
    if (sock_fd < 0) {
        write(2, "socket() failed\n", 16);
        return;
    }

    ret = connect(sock_fd, (struct sockaddr *)&servaddr, SUN_LEN(&servaddr));
    if (ret < 0) {
        write(2, "socket() failed\n", 16);
        return;
    }

    write(1, "Sock Connected\n", 15);

    send(sock_fd, "Hello from constructor\n", 24, 0);
}
