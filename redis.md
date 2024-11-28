```
import org.springframework.stereotype.Component;
import org.springframework.web.filter.OncePerRequestFilter;
import redis.clients.jedis.Jedis;
import javax.servlet.FilterChain;
import javax.servlet.FilterConfig;
import javax.servlet.ServletException;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.concurrent.TimeUnit;

@Component
public class RateLimitingFilter extends OncePerRequestFilter {

    private static final int MAX_REQUESTS_PER_MINUTE = 5;
    private static final String REDIS_HOST = "localhost";
    private static final int REDIS_PORT = 6379;

    @Override
    protected void doFilterInternal(HttpServletRequest request, HttpServletResponse response, FilterChain filterChain)
            throws ServletException, IOException {

        String clientIp = request.getRemoteAddr(); // Identify client by IP
        String key = "rate_limit:" + clientIp;

        try (Jedis jedis = new Jedis(REDIS_HOST, REDIS_PORT)) {

            String requestCount = jedis.get(key);

            if (requestCount == null) {
                // First request, set count to 1 and set expiration for 1 minute
                jedis.setex(key, 60, "1");
            } else {
                int currentCount = Integer.parseInt(requestCount);

                if (currentCount < MAX_REQUESTS_PER_MINUTE) {
                    // Increment the request count
                    jedis.incr(key);
                } else {
                    // Limit exceeded, send 429 Too Many Requests response
                    response.setStatus(HttpServletResponse.SC_TOO_MANY_REQUESTS);
                    response.getWriter().write("Rate limit exceeded. Try again later.");
                    return;
                }
            }
        }

        filterChain.doFilter(request, response); // Continue the request
    }

    @Override
    public void init(FilterConfig filterConfig) throws ServletException {
        // Optional: initialize resources
    }

    @Override
    public void destroy() {
        // Optional: clean up resources
    }
}
```
